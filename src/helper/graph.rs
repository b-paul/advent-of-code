use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// A type with read only local directed graph structure, where we don't necessarily get a way to
/// find all vertices, but we can compute adjacency.
pub trait DigraphView<V, E> {
    fn adj(&self, point: V) -> Option<impl Iterator<Item = (E, V)>>;

    /// Creates a new graph where trivial edges are collapsed (think o-o-o-o turning into o-o) and
    /// structure is encoded in a new edge type.
    fn collapse_with<E2, F1, F2>(&self, start: V, map: F1, join: F2) -> EdgeMap<V, E2>
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

        while let Some(mut from) = stack.pop() {
            for (e, mut to) in self.adj(from) {
                todo!()
            }
        }

        EdgeMap { map }
    }
}

pub struct EdgeMap<V, E> {
    map: HashMap<V, Vec<(E, V)>>,
}
