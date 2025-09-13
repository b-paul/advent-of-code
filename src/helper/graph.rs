use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// A type with read only local directed graph structure, where we don't necessarily get a way to
/// find all vertices, but we can compute adjacency.
pub trait DigraphView<V, E> {
    fn adj(&self, point: V) -> Option<impl Iterator<Item = (E, V)>>;

    /// Creates a new graph where trivial edges are collapsed (think o-o-o-o turning into o-o) and
    /// structure is encoded in a new edge type.
    fn collapse_with<E2, F1, F2>(&self, start: V, convert: F1, join: F2) -> EdgeMap<V, E2>
    where
        V: Copy + Eq + Hash,
        F1: Fn(E) -> E2,
        F2: Fn(E2, E2) -> E2,
    {
        let mut map = HashMap::new();
        map.insert(start, Vec::new());

        let mut stack = Vec::new();
        let mut visited = HashSet::new();
        stack.push(start);
        visited.insert(start);

        while let Some(from) = stack.pop() {
            for (e, mut to) in self.adj(from).into_iter().flatten() {
                let mut e = convert(e);
                while let Some((e2, to2)) = self
                    .adj(to)
                    .and_then(|adjs| adjs.filter(|&(_, v)| v != from).exactly_one().ok())
                {
                    to = to2;
                    e = join(e, convert(e2));
                }
                map.entry(from).or_default().push((e, to));
                if !visited.contains(&to) {
                    stack.push(to);
                    visited.insert(to);
                }
            }
        }

        EdgeMap { map }
    }

    /// Collapse trivial edges in a graph (o-o-o into o-o) and count how many edges are collapsed.
    fn collapse_count(&self, start: V) -> EdgeMap<V, usize>
    where
        V: Copy + Eq + Hash,
    {
        self.collapse_with(start, |_| 1, |a, b| a + b)
    }
}

pub struct EdgeMap<V, E> {
    map: HashMap<V, Vec<(E, V)>>,
}
