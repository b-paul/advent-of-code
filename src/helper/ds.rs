//! General data structures

use std::collections::HashMap;
use std::hash::Hash;

/// A union find data structure for representing and keeping track of sizes of equivalence classes
pub struct UnionFind<T: Eq + Hash + Clone> {
    heads: HashMap<T, T>,
    components: HashMap<T, usize>,
}

impl<T: Eq + Hash + Clone> UnionFind<T> {
    pub fn new(vals: impl IntoIterator<Item = T>) -> Self {
        let mut heads = HashMap::new();
        let mut components = HashMap::new();

        for v in vals {
            components.insert(v.clone(), 1);
            heads.insert(v.clone(), v);
        }

        Self { heads, components }
    }

    /// Obtain the representative of x.
    pub fn find(&mut self, x: T) -> T {
        let mut x2 = x.clone();
        while self.heads[&x2] != x2 {
            x2 = self.heads[&x2].clone();
        }
        let mut x3 = x;
        while self.heads[&x3] != x2 {
            let t = self.heads[&x3].clone();
            self.heads.insert(x3, x2.clone());
            x3 = t
        }
        x2
    }

    /// Join two classes containing the given points
    pub fn join(&mut self, a: T, b: T) {
        let a2 = self.find(a);
        let b2 = self.find(b);
        if b2 != a2 {
            self.components
                .insert(a2.clone(), self.components[&a2] + self.components[&b2]);
            self.components.remove(&b2);
            self.heads.insert(b2, a2);
        }
    }

    /// Returns the number of components
    pub fn len(&self) -> usize {
        self.components.len()
    }

    /// Iteratate over representatives of classes
    pub fn classes(self) -> impl Iterator<Item = T> {
        self.heads.into_values()
    }

    /// Iterate over sizes of classes
    pub fn sizes(self) -> impl Iterator<Item = usize> {
        self.components.into_values()
    }

    /// Iterate over representative,size pairs of the classes
    pub fn classes_s(self) -> impl Iterator<Item = (T, usize)> {
        let mut m = HashMap::new();
        for k in self.heads.keys() {
            m.insert(k.clone(), (self.heads[k].clone(), self.components[k]));
        }
        m.into_values()
    }
}
