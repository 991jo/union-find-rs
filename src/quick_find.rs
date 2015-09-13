use std::iter::FromIterator;
use std::mem;
use {UfValue, UnionFind, Merge};

/// Union-Find implementation with quick find operation.
#[derive(Clone, Debug)]
pub struct QuickFindUf<V> {
    parent: Vec<usize>,
    data: Vec<Option<V>>
}

impl<V: UfValue> UnionFind<V> for QuickFindUf<V> {
    /// Returns the size of `self`.
    #[inline]
    fn size(&self) -> usize { self.data.len() }

    /// Join two sets that contains given keys (Union operation).
    ///
    /// Returns `true` if these keys are belonged to different sets.
    #[inline]
    fn union(&mut self, key0: usize, key1: usize) -> bool {
        let k0 = self.find(key0);
        let k1 = self.find(key1);
        if k0 == k1 { return false; }

        // Temporary replace with dummy to move out the elements of the vector.
        let v0 = mem::replace(&mut self.data[k0], None).unwrap();
        let v1 = mem::replace(&mut self.data[k1], None).unwrap();

        let (parent, child, val) = match UfValue::merge(v0, v1) {
            Merge::Left(val) => (k0, k1, val),
            Merge::Right(val) => (k1, k0, val)
        };
        self.data[parent] = Some(val);

        for p in &mut self.parent {
            if *p == child {
                *p = parent;
            }
        }

        true
    }

    /// Returns the identifier of the set that the key belongs to.
    #[inline]
    fn find(&mut self, key: usize) -> usize { self.parent[key] }

    /// Returns the reference to the value of the set that the key belongs to.
    #[inline]
    fn get(&mut self, key: usize) -> &V {
        let root_key = self.find(key);
        self.data[root_key].as_ref().unwrap()
    }

    /// Returns the mutable reference to the value of the set that the key belongs to.
    #[inline]
    fn get_mut(&mut self, key: usize) -> &mut V {
        let root_key = self.find(key);
        self.data[root_key].as_mut().unwrap()
    }
}

impl<A: UfValue> FromIterator<A> for QuickFindUf<A> {
    #[inline]
    fn from_iter<T: IntoIterator<Item=A>>(iterator: T) -> QuickFindUf<A> {
        let data = iterator.into_iter().map(Some).collect::<Vec<_>>();
        QuickFindUf {
            parent: (0..data.len()).collect(),
            data: data
        }
    }
}
