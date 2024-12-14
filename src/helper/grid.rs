use crate::helper::adjacency::{
    adjacent_4_ud, adjacent_8_ud, move_off, Direction, Direction4, Direction8,
};
use crate::helper::point::{Bounds, Offset, Point};
use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::{Index, IndexMut};
use std::str::FromStr;
use thiserror::Error;

// TODO bfs/dfs_mut
// TODO impl Debug
// TODO insert row/col
// maybe want a deque like grid or something :grimacing:
// Move GridPoint by offset or by direction
// Trajectory for length n at point p in direction dir
// TODO some sort of spare grid, or a sparse way to iterate over a grid somehow.
// TODO owning iterator (and refactor functions to use this such as map and map_i)
// TODO scanning in a direction iterator

pub struct Grid<T> {
    entries: Vec<T>,
    bound: Bounds,
}

impl<T> Grid<T> {
    pub fn width(&self) -> usize {
        self.bound.width
    }

    pub fn height(&self) -> usize {
        self.bound.height
    }

    pub fn bounds(&self) -> Bounds {
        self.bound
    }

    /// returns (VecVec, width, height)
    pub fn into_vecvec(self) -> (Vec<Vec<T>>, Bounds) {
        let mut grid = Vec::new();
        let mut line = Vec::new();
        for (i, x) in self.entries.into_iter().enumerate() {
            line.push(x);
            if i % self.bound.width == self.bound.width - 1 {
                grid.push(line);
                line = Vec::new();
            }
        }
        (grid, self.bound)
    }

    pub fn map<F, U>(self, f: F) -> Grid<U>
    where
        F: FnMut(T) -> U,
    {
        let bound = self.bound;
        let entries = self.entries.into_iter().map(f).collect();
        Grid { entries, bound }
    }

    /// Apply a function to every cell of the grid, where the function additionally takes the point
    /// of the cell as an input.
    pub fn map_i<F, U>(self, mut f: F) -> Grid<U>
    where
        F: FnMut(Point, T) -> U,
    {
        let bound = self.bound;
        let entries = self
            .entries
            .into_iter()
            .enumerate()
            .map(|(i, v)| {
                f(
                    Point {
                        x: i % bound.width,
                        y: i / bound.width,
                    },
                    v,
                )
            })
            .collect();
        Grid { entries, bound }
    }

    pub fn iter_idx(&self) -> GridIdxIterator<T> {
        GridIdxIterator {
            grid: self,
            cur: Point { x: 0, y: 0 },
        }
    }

    pub fn contains_point(&self, p: Point) -> bool {
        self.bound.contains_point(p)
    }

    pub fn iter_rows(&self) -> GridRowIter<T> {
        GridRowIter {
            grid: self,
            index: 0,
        }
    }

    pub fn iter_cols(&self) -> GridColIter<T> {
        GridColIter {
            grid: self,
            index: 0,
        }
    }

    pub fn into_vec(self) -> Vec<T> {
        self.entries
    }

    pub fn get(&self, idx: Point) -> Option<&T> {
        self.contains_point(idx).then(|| &self[idx])
    }

    /// Return the entry at the given point.
    pub fn point(&self, pos: Point) -> Option<GridEntry<T>> {
        self.contains_point(pos).then_some(GridEntry {
            pos,
            grid: self,
        })
    }
}

impl<T: Copy + Eq + Hash> Grid<T> {
    /// F takes a point, entry
    /// P takes a direction, from and to
    pub fn dfs_4<F, P>(&self, start: Point, mut f: F, p: P)
    where
        F: FnMut(Point, T),
        P: Fn(Direction4, T, T) -> bool,
    {
        let mut stack = Vec::new();
        let mut visited = HashSet::new();
        stack.push(start);
        visited.insert(start);

        while let Some(from) = stack.pop() {
            f(from, self[from]);
            for (to, dir) in adjacent_4_ud(from.x, from.y) {
                if self.contains_point(to)
                    && p(dir, self[from], self[to])
                    && !visited.contains(&to)
                {
                    stack.push(to);
                    visited.insert(to);
                }
            }
        }
    }

    /// F takes a point, entry
    /// P takes a direction, from and to
    /// Allows dups (was made for 2024 day 10 part 2 lol)
    pub fn dfs_4_dups<F, P>(&self, start: Point, mut f: F, p: P)
    where
        F: FnMut(Point, T),
        P: Fn(Direction4, T, T) -> bool,
    {
        let mut stack = Vec::new();
        stack.push(start);

        while let Some(from) = stack.pop() {
            f(from, self[from]);
            for (to, dir) in adjacent_4_ud(from.x, from.y) {
                if self.contains_point(to)
                    && p(dir, self[from], self[to])
                {
                    stack.push(to);
                }
            }
        }
    }

    /// F takes a point, entry
    /// P takes a direction, from and to
    pub fn dfs_8<F, P>(&self, start: Point, mut f: F, p: P)
    where
        F: FnMut(Point, T),
        P: Fn(Direction8, T, T) -> bool,
    {
        let mut stack = Vec::new();
        let mut visited = HashSet::new();
        stack.push(start);
        visited.insert(start);

        while let Some(from) = stack.pop() {
            f(from, self[from]);
            for (to, dir) in adjacent_8_ud(from.x, from.y) {
                if self.contains_point(to)
                    && p(dir, self[from], self[to])
                    && !visited.contains(&to)
                {
                    stack.push(to);
                    visited.insert(to);
                }
            }
        }
    }

    /// F takes a point, entry, depth
    /// P takes a direction, from and to
    pub fn bfs_4<F, P>(&self, start: Point, mut f: F, p: P)
    where
        F: FnMut(Point, T, usize),
        P: Fn(Direction4, T, T) -> bool,
    {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((start, 0));
        visited.insert(start);

        while let Some((from, depth)) = queue.pop_front() {
            f(from, self[from], depth);
            for (to, dir) in adjacent_4_ud(from.x, from.y) {
                if self.contains_point(to)
                    && p(dir, self[from], self[to])
                    && !visited.contains(&to)
                {
                    queue.push_back((to, depth + 1));
                    visited.insert(to);
                }
            }
        }
    }

    /// F takes a point, entry, depth
    /// P takes a direction, from and to
    pub fn bfs_8<F, P>(&self, start: Point, mut f: F, p: P)
    where
        F: FnMut(Point, T, usize),
        P: Fn(Direction8, T, T) -> bool,
    {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((start, 0));
        visited.insert(start);

        while let Some((from, depth)) = queue.pop_front() {
            f(from, self[from], depth);
            for (to, dir) in adjacent_8_ud(from.x, from.y) {
                if self.contains_point(to)
                    && p(dir, self[from], self[to])
                    && !visited.contains(&to)
                {
                    queue.push_back((to, depth + 1));
                    visited.insert(to);
                }
            }
        }
    }
}

impl<T: Copy> Grid<T> {
    pub fn new_filled(value: T, bound: Bounds) -> Grid<T> {
        Grid {
            entries: vec![value; bound.width * bound.height],
            bound,
        }
    }

    pub fn set_border(&mut self, val: T) {
        let w = self.bound.width;
        let h = self.bound.height;
        for x in 0..w {
            self[(x, 0)] = val;
            self[(x, h - 1)] = val;
        }
        for y in 0..h {
            self[(0, y)] = val;
            self[(w - 1, y)] = val;
        }
    }

    pub fn get_row(&self, index: usize) -> Option<Vec<T>> {
        if index >= self.bound.height {
            None
        } else {
            let start = index * self.bound.width;
            Some(self.entries[start..start + self.bound.width].to_vec())
        }
    }

    pub fn get_col(&self, index: usize) -> Option<Vec<T>> {
        if index >= self.bound.width {
            None
        } else {
            let mut col = Vec::new();
            let mut i = index;
            for _ in 0..self.bound.height {
                col.push(self.entries[i]);
                i += self.bound.width;
            }
            Some(col)
        }
    }

    pub fn transpose(&self) -> Grid<T> {
        let mut entries = Vec::with_capacity(self.bound.width * self.bound.height);
        let bound = Bounds {
            width: self.bound.height,
            height: self.bound.width,
        };
        for col in self.iter_cols() {
            for x in col {
                entries.push(*x);
            }
        }
        Grid { entries, bound }
    }

    // TODO should this really take an &self ???
    pub fn rotate_cw(&self) -> Grid<T> {
        let mut entries = Vec::with_capacity(self.bound.width * self.bound.height);
        let bound = Bounds {
            width: self.bound.height,
            height: self.bound.width,
        };
        for y in 0..self.bound.width {
            for x in 0..self.bound.height {
                entries.push(self[(y, self.bound.width - x - 1)]);
            }
        }

        Grid { entries, bound }
    }

    pub fn rotate_acw(&self) -> Grid<T> {
        let mut entries = Vec::with_capacity(self.bound.width * self.bound.height);
        let bound = Bounds {
            width: self.bound.height,
            height: self.bound.width,
        };
        for y in 0..self.bound.width {
            for x in 0..self.bound.height {
                entries.push(self[(self.bound.height - y - 1, x)]);
            }
        }

        Grid { entries, bound }
    }
}

impl<T: Clone> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Self {
            entries: self.entries.clone(),
            bound: self.bound,
        }
    }
}

impl<T: PartialEq> PartialEq for Grid<T> {
    fn eq(&self, other: &Self) -> bool {
        self.entries == other.entries && self.bound == other.bound
    }
}

impl<T: Eq> Eq for Grid<T> {}

impl<T: PartialEq> Grid<T> {
    pub fn find(&self, elem: &T) -> Option<Point> {
        self.iter_idx()
            .filter_map(|(p, val)| (val == elem).then_some(p))
            .next()
    }
}

impl<T: PartialEq + Copy> Grid<T> {
    /// start must be in the bounds of the grid
    pub fn floodfill_4(&mut self, empty: T, with: T, start: Point) {
        let mut set = HashSet::new();
        let mut stack = Vec::new();
        stack.push(start);
        set.insert(start);
        while let Some(p) = stack.pop() {
            self[p] = with;
            for p in super::adjacency::adjacent_4_u(p.x, p.y) {
                if !self.contains_point(p) {
                    continue;
                }
                if self[p] == empty && !set.contains(&p) {
                    set.insert(p);
                    stack.push(p);
                }
            }
        }
    }
}

impl Display for Grid<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.bound.height {
            for x in 0..self.bound.width {
                write!(f, "{}", self[(x, y)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Grid")
            .field("entries", &self.entries)
            .field("bound", &self.bound)
            .finish()
    }
}

#[derive(Debug, Error)]
pub enum GridParseError {
    #[error("The provided input did not have a straight edge")]
    NotRectangle,
}

/// This isn't really perf optimized, if you want a fast implementation i'll do a byte one probably
impl FromStr for Grid<char> {
    type Err = GridParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut entries = Vec::new();
        let mut height = 0;
        let mut width = 0;
        for line in s.lines() {
            if width == 0 {
                width = line.len();
            } else if width != line.len() {
                return Err(GridParseError::NotRectangle);
            }
            for c in line.chars() {
                entries.push(c);
            }
            height += 1;
        }
        let bound = Bounds { width, height };

        Ok(Grid { entries, bound })
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        let p = Point { x, y };
        assert!(
            self.contains_point(p),
            "index out of bounds: (width,height) = ({},{}) but index is ({},{})",
            self.bound.width,
            self.bound.height,
            x,
            y
        );
        &self.entries[y * self.bound.width + x]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        let p = Point { x, y };
        assert!(
            self.contains_point(p),
            "index out of bounds: (width,height) = ({},{}) but index is ({},{})",
            self.bound.width,
            self.bound.height,
            x,
            y
        );
        self.entries.index_mut(y * self.bound.width + x)
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, p: Point) -> &Self::Output {
        self.index(p.pair())
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, p: Point) -> &mut Self::Output {
        self.index_mut(p.pair())
    }
}

impl<T: Hash> Hash for Grid<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.entries.hash(state);
        self.bound.hash(state);
    }
}

/// A "pointer into a cell in a grid"
#[derive(Clone, Copy)]
pub struct GridEntry<'a, T> {
    /// The cell we are pointin at
    pos: Point,
    /// The grid
    grid: &'a Grid<T>,
}

impl<'a, T> GridEntry<'a, T> {
    /// Get the value at this point on the grid.
    pub fn val(&self) -> &T {
        self.grid
            .get(self.pos)
            .expect("Invalid GridEntry was created")
    }

    /// Get the position of this point on the grid.
    pub fn pos(&self) -> Point {
        self.pos
    }

    /// Get the entry at (pos + offset) on the grid, if it is in bounds.
    pub fn move_off(&self, offset: Offset) -> Option<GridEntry<'a, T>> {
        self.pos.move_off(offset).map(|pos| GridEntry {
            pos,
            grid: self.grid,
        })
    }

    /// Get the point at the position found by moving in the direction dir, if it is in bounds.
    pub fn move_dir<D: Direction>(&self, dir: D) -> Option<GridEntry<'a, T>> {
        let pos = dir.moveu(self.pos)?;
        self.grid.contains_point(pos).then_some(GridEntry {
            pos,
            grid: self.grid,
        })
    }

    /// Get the point at the position found by moving in the direction dir c times, if it is in
    /// bounds.
    pub fn move_dirc<D: Direction>(&self, dir: D, c: usize) -> Option<GridEntry<'a, T>> {
        let pos = dir.moveuc(self.pos, c)?;
        self.grid.contains_point(pos).then_some(GridEntry {
            pos,
            grid: self.grid,
        })
    }

    /// Returns an iterator of entries into the grid found by moving by the offset off for len
    /// steps. We count this current point as step 0, meaning that if len = 1, we will not move to
    /// another point.
    pub fn trajectory(&self, off: Offset, len: usize) -> Trajectory<'a, T> {
        Trajectory {
            pos: self.pos,
            steps: len,
            off,
            grid: self.grid,
        }
    }

    /// Returns an iterator of entries into the grid found by following the direction dir len
    /// steps. We count this current point as step 0, meaning that if len = 1, we will not move to
    /// another point.
    pub fn trajectory_dir<D: Direction>(&self, dir: D, len: usize) -> Trajectory<'a, T> {
        Trajectory {
            pos: self.pos,
            steps: len,
            off: dir.offset(),
            grid: self.grid,
        }
    }

    // TODO call these bounded trajectories, have unbounded trajectories.
}

impl<T: Debug> Debug for GridEntry<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GridEntry")
            .field("pos", &self.pos)
            .field("grid", &self.grid)
            .finish()
    }
}

#[derive(Clone)]
pub struct GridIdxIterator<'a, T> {
    grid: &'a Grid<T>,
    cur: Point,
}

impl<'a, T> Iterator for GridIdxIterator<'a, T> {
    type Item = (Point, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let p = self.cur;
        self.cur.x += 1;
        if self.cur.x >= self.grid.bound.width {
            self.cur.x = 0;
            self.cur.y += 1;
        }
        if p.y >= self.grid.bound.height {
            None
        } else {
            Some((p, &self.grid[p]))
        }
    }
}

#[derive(Clone)]
pub struct Row<'a, T> {
    grid: &'a Grid<T>,
    index: usize,
    end: usize,
}

impl<'a, T> Iterator for Row<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.end {
            None
        } else {
            let val = &self.grid.entries[self.index];
            self.index += 1;
            Some(val)
        }
    }
}

#[derive(Clone)]
pub struct GridRowIter<'a, T> {
    grid: &'a Grid<T>,
    index: usize,
}

impl<'a, T> Iterator for GridRowIter<'a, T> {
    type Item = Row<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.grid.bound.height {
            None
        } else {
            let row = Row {
                grid: self.grid,
                index: self.index * self.grid.bound.width,
                end: (self.index + 1) * self.grid.bound.width - 1,
            };
            self.index += 1;
            Some(row)
        }
    }
}

#[derive(Clone)]
pub struct Col<'a, T> {
    grid: &'a Grid<T>,
    index: usize,
}

impl<'a, T> Iterator for Col<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.grid.entries.len() {
            None
        } else {
            let val = &self.grid.entries[self.index];
            self.index += self.grid.bound.width;
            Some(val)
        }
    }
}

#[derive(Clone)]
pub struct GridColIter<'a, T> {
    grid: &'a Grid<T>,
    index: usize,
}

impl<'a, T> Iterator for GridColIter<'a, T> {
    type Item = Col<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.grid.bound.width {
            None
        } else {
            let col = Col {
                grid: self.grid,
                index: self.index,
            };
            self.index += 1;
            Some(col)
        }
    }
}

#[derive(Clone)]
pub struct Trajectory<'a, T> {
    pos: Point,
    /// The number of values left to return
    steps: usize,
    off: Offset,
    grid: &'a Grid<T>,
}

impl<'a, T> Iterator for Trajectory<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.steps == 0 {
            None
        } else {
            let val = self.grid.get(self.pos)?;
            // Move to the next position, but if it is invalid then set steps to 0 (so that we
            // always return None)
            self.steps -= 1;
            self.pos = match self.pos.move_off(self.off) {
                Some(p) => p,
                None => {
                    self.steps = 0;
                    self.pos
                }
            };
            Some(val)
        }
    }
}
