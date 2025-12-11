use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::Hash;
use std::io::Write;
use std::path::Path;

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

        // BUG this algorithm collapses edges when they have trivial out edge relations, but does
        // not respect when edges have non-trivial edge in relations!

        let mut stack = Vec::new();
        let mut visited = HashSet::new();
        stack.push(start);
        visited.insert(start);

        while let Some(from) = stack.pop() {
            for (e, mut to) in self.adj(from).into_iter().flatten() {
                let mut prev = from;
                let mut e = convert(e);
                while let Some((e2, to2)) = self
                    .adj(to)
                    .and_then(|adjs| adjs.filter(|&(_, v)| v != prev).exactly_one().ok())
                {
                    prev = to;
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

    fn write_graphviz<P: AsRef<Path>>(
        &self,
        basepoints: impl Iterator<Item = V>,
        path: P,
    ) -> std::io::Result<()>
    where
        V: Copy + Eq + Hash + std::fmt::Debug,
        E: std::fmt::Debug,
    {
        let mut stack: Vec<_> = basepoints.collect();
        let mut vertices: HashSet<_> = stack.clone().into_iter().collect();

        while let Some(next) = stack.pop() {
            vertices.insert(next);
            for (_, adj) in self.adj(next).into_iter().flatten() {
                if !vertices.contains(&adj) {
                    stack.push(adj);
                    vertices.insert(next);
                }
            }
        }

        let mut f = File::create(path)?;

        writeln!(&mut f, "digraph {{")?;
        for v in vertices {
            for (e, v2) in self.adj(v).into_iter().flatten() {
                writeln!(&mut f, "\"{v:?}\" -> \"{v2:?}\"[label={e:?}]")?;
            }
        }
        writeln!(&mut f, "}}")?;

        Ok(())
    }

    fn write_graphviz_undirected<P: AsRef<Path>>(
        &self,
        basepoints: impl Iterator<Item = V>,
        path: P,
    ) -> std::io::Result<()>
    where
        V: Copy + Eq + Hash + std::fmt::Display,
        E: std::fmt::Debug,
    {
        let mut stack: Vec<_> = basepoints.collect();
        let mut vertices: HashSet<_> = stack.clone().into_iter().collect();

        while let Some(next) = stack.pop() {
            vertices.insert(next);
            for (_, adj) in self.adj(next).into_iter().flatten() {
                if !vertices.contains(&adj) {
                    stack.push(adj);
                    vertices.insert(next);
                }
            }
        }

        let mut f = File::create(path)?;

        writeln!(&mut f, "graph {{")?;
        let mut drawn = HashSet::new();
        for v in vertices {
            for (e, v2) in self.adj(v).into_iter().flatten() {
                if !drawn.contains(&(v2, v)) {
                    drawn.insert((v, v2));
                    writeln!(&mut f, "\"{v}\" -- \"{v2}\"[label={e:?}]")?;
                }
            }
        }
        writeln!(&mut f, "}}")?;

        Ok(())
    }
}

pub struct EdgeMap<V, E> {
    pub map: HashMap<V, Vec<(E, V)>>,
}

impl<V, E> DigraphView<V, E> for EdgeMap<V, E>
where
    V: Hash + Eq + Clone,
    E: Clone,
{
    fn adj(&self, point: V) -> Option<impl Iterator<Item = (E, V)>> {
        self.map.get(&point).map(|v| v.iter().cloned())
    }
}
