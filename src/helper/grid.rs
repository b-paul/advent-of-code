use crate::helper::adjacency::{adjacent_4_ud, adjacent_8_ud, Direction4, Direction8};
use std::collections::{HashSet, VecDeque};
use std::fmt::Display;
use std::hash::Hash;
use std::ops::{Index, IndexMut};
use std::str::FromStr;
use thiserror::Error;

// TODO bfs/dfs_mut
// TODO impl Debug
// TODO insert row/col
// maybe want a deque like grid or something :grimacing:

pub struct Grid<T> {
    entries: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    /// returns (VecVec, width, height)
    pub fn into_vecvec(self) -> (Vec<Vec<T>>, usize, usize) {
        let mut grid = Vec::new();
        let mut line = Vec::new();
        for (i, x) in self.entries.into_iter().enumerate() {
            line.push(x);
            if i % self.width == self.width - 1 {
                grid.push(line);
                line = Vec::new();
            }
        }
        (grid, self.width, self.height)
    }

    pub fn map<F, U>(self, f: F) -> Grid<U>
    where
        F: FnMut(T) -> U,
    {
        let width = self.width;
        let height = self.height;
        let entries = self.entries.into_iter().map(f).collect();
        Grid {
            entries,
            width,
            height,
        }
    }

    pub fn map_i<F, U>(self, mut f: F) -> Grid<U>
    where
        F: FnMut((usize, usize), T) -> U,
    {
        let width = self.width;
        let height = self.height;
        let entries = self
            .entries
            .into_iter()
            .enumerate()
            .map(|(i, x)| f((i % width, i / width), x))
            .collect();
        Grid {
            entries,
            width,
            height,
        }
    }

    pub fn iter_idx(&self) -> GridIdxIterator<T> {
        GridIdxIterator {
            grid: self,
            cur_x: 0,
            cur_y: 0,
        }
    }

    pub fn contains_point(&self, (x, y): (usize, usize)) -> bool {
        x < self.width && y < self.height
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

    pub fn to_vec(self) -> Vec<T> {
        self.entries
    }

    pub fn get(&self, idx: (usize, usize)) -> Option<&T> {
        self.contains_point(idx).then(|| &self[idx])
    }
}

impl<T: Copy + Eq + Hash> Grid<T> {
    /// F takes a point, entry
    /// P takes a direction, from and to
    pub fn dfs_4<F, P>(&self, start: (usize, usize), mut f: F, p: P)
    where
        F: FnMut((usize, usize), T),
        P: Fn(Direction4, T, T) -> bool,
    {
        let mut stack = Vec::new();
        let mut visited = HashSet::new();
        stack.push(start);
        visited.insert(start);

        while let Some(from) = stack.pop() {
            f(from, self[from]);
            for (to, dir) in adjacent_4_ud(from.0, from.1) {
                if self.contains_point(to) && p(dir, self[from], self[to]) && !visited.contains(&to)
                {
                    stack.push(to);
                    visited.insert(to);
                }
            }
        }
    }

    /// F takes a point, entry
    /// P takes a direction, from and to
    pub fn dfs_8<F, P>(&self, start: (usize, usize), mut f: F, p: P)
    where
        F: FnMut((usize, usize), T),
        P: Fn(Direction8, T, T) -> bool,
    {
        let mut stack = Vec::new();
        let mut visited = HashSet::new();
        stack.push(start);
        visited.insert(start);

        while let Some(from) = stack.pop() {
            f(from, self[from]);
            for (to, dir) in adjacent_8_ud(from.0, from.1) {
                if self.contains_point(to) && p(dir, self[from], self[to]) && !visited.contains(&to)
                {
                    stack.push(to);
                    visited.insert(to);
                }
            }
        }
    }

    /// F takes a point, entry, depth
    /// P takes a direction, from and to
    pub fn bfs_4<F, P>(&self, start: (usize, usize), mut f: F, p: P)
    where
        F: FnMut((usize, usize), T, usize),
        P: Fn(Direction4, T, T) -> bool,
    {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((start, 0));
        visited.insert(start);

        while let Some((from, depth)) = queue.pop_front() {
            f(from, self[from], depth);
            for (to, dir) in adjacent_4_ud(from.0, from.1) {
                if self.contains_point(to) && p(dir, self[from], self[to]) && !visited.contains(&to)
                {
                    queue.push_back((to, depth + 1));
                    visited.insert(to);
                }
            }
        }
    }

    /// F takes a point, entry, depth
    /// P takes a direction, from and to
    pub fn bfs_8<F, P>(&self, start: (usize, usize), mut f: F, p: P)
    where
        F: FnMut((usize, usize), T, usize),
        P: Fn(Direction8, T, T) -> bool,
    {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((start, 0));
        visited.insert(start);

        while let Some((from, depth)) = queue.pop_front() {
            f(from, self[from], depth);
            for (to, dir) in adjacent_8_ud(from.0, from.1) {
                if self.contains_point(to) && p(dir, self[from], self[to]) && !visited.contains(&to)
                {
                    queue.push_back((to, depth + 1));
                    visited.insert(to);
                }
            }
        }
    }
}

impl<T: Copy> Grid<T> {
    pub fn new_filled(value: T, width: usize, height: usize) -> Grid<T> {
        Grid {
            entries: vec![value; width * height],
            width,
            height,
        }
    }

    pub fn set_border(&mut self, val: T) {
        let w = self.width;
        let h = self.height;
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
        if index >= self.height {
            None
        } else {
            let start = index * self.width;
            Some(self.entries[start..start + self.width].to_vec())
        }
    }

    pub fn get_col(&self, index: usize) -> Option<Vec<T>> {
        if index >= self.width {
            None
        } else {
            let mut col = Vec::new();
            let mut i = index;
            for _ in 0..self.height {
                col.push(self.entries[i]);
                i += self.width;
            }
            Some(col)
        }
    }

    pub fn transpose(&self) -> Grid<T> {
        let mut entries = Vec::with_capacity(self.width * self.height);
        for col in self.iter_cols() {
            for x in col {
                entries.push(*x);
            }
        }
        Grid {
            entries,
            width: self.height,
            height: self.width,
        }
    }

    pub fn rotate_cw(&self) -> Grid<T> {
        let mut entries = Vec::with_capacity(self.width * self.height);

        for y in 0..self.width {
            for x in 0..self.height {
                entries.push(self[(y, self.width - x - 1)]);
            }
        }

        Grid {
            entries,
            width: self.height,
            height: self.width,
        }
    }

    pub fn rotate_acw(&self) -> Grid<T> {
        let mut entries = Vec::with_capacity(self.width * self.height);

        for y in 0..self.width {
            for x in 0..self.height {
                entries.push(self[(self.height - y - 1, x)]);
            }
        }

        Grid {
            entries,
            width: self.height,
            height: self.width,
        }
    }
}

impl<T: Clone> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Self {
            entries: self.entries.clone(),
            width: self.width,
            height: self.height,
        }
    }
}

impl<T: PartialEq> PartialEq for Grid<T> {
    fn eq(&self, other: &Self) -> bool {
        self.entries == other.entries && self.width == other.width && self.height == other.height
    }
}

impl<T: Eq> Eq for Grid<T> {}

impl<T: PartialEq> Grid<T> {
    pub fn find(&self, elem: &T) -> Option<(usize, usize)> {
        for y in 0..self.height {
            for x in 0..self.width {
                if self[(x, y)] == *elem {
                    return Some((x, y));
                }
            }
        }
        None
    }
}

impl<T: PartialEq + Copy> Grid<T> {
    /// start must be in the bounds of the grid
    pub fn floodfill_4(&mut self, empty: T, with: T, start: (usize, usize)) {
        let mut set = HashSet::new();
        let mut stack = Vec::new();
        stack.push(start);
        set.insert(start);
        while let Some((x, y)) = stack.pop() {
            self[(x, y)] = with;
            for p in super::adjacency::adjacent_4_u(x, y) {
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
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self[(x, y)])?;
            }
            writeln!(f)?;
        }
        Ok(())
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

        Ok(Grid {
            entries,
            width,
            height,
        })
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        assert!(
            x < self.width && y < self.height,
            "index out of bounds: (width,height) = ({},{}) but index is ({},{})",
            self.width,
            self.height,
            x,
            y
        );
        &self.entries[y * self.width + x]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        assert!(
            x < self.width && y < self.height,
            "index out of bounds: (width,height) = ({},{}) but index is ({},{})",
            self.width,
            self.height,
            x,
            y
        );
        self.entries.index_mut(y * self.width + x)
    }
}

impl<T: Hash> Hash for Grid<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.entries.hash(state);
        self.width.hash(state);
        self.height.hash(state);
    }
}

pub struct GridIdxIterator<'a, T> {
    grid: &'a Grid<T>,
    cur_x: usize,
    cur_y: usize,
}

impl<'a, T> Iterator for GridIdxIterator<'a, T> {
    type Item = ((usize, usize), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.cur_x;
        let y = self.cur_y;
        self.cur_x += 1;
        if self.cur_x >= self.grid.width {
            self.cur_x = 0;
            self.cur_y += 1;
        }
        if y >= self.grid.height {
            None
        } else {
            Some(((x, y), &self.grid[(x, y)]))
        }
    }
}

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

pub struct GridRowIter<'a, T> {
    grid: &'a Grid<T>,
    index: usize,
}

impl<'a, T> Iterator for GridRowIter<'a, T> {
    type Item = Row<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.grid.height {
            None
        } else {
            let row = Row {
                grid: self.grid,
                index: self.index * self.grid.width,
                end: (self.index + 1) * self.grid.width - 1,
            };
            self.index += 1;
            Some(row)
        }
    }
}

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
            self.index += self.grid.width;
            Some(val)
        }
    }
}

pub struct GridColIter<'a, T> {
    grid: &'a Grid<T>,
    index: usize,
}

impl<'a, T> Iterator for GridColIter<'a, T> {
    type Item = Col<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.grid.width {
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
