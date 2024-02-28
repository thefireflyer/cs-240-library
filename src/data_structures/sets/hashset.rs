///////////////////////////////////////////////////////////////////////////////

use std::fmt;

use crate::data_structures::linked_list::single_linked_list::solution::LinkedList;

///////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, PartialOrd, Debug)]
pub struct HashSet<T>
where
    T: Ord + fmt::Debug,
    Vec<u8>: From<T>,
{
    inner: Vec<LinkedList<T>>,
    len: usize,
    radix: usize,
}

//---------------------------------------------------------------------------//

impl<T> HashSet<T>
where
    T: Ord + fmt::Debug + Clone,
    Vec<u8>: From<T>,
{
    pub fn new() -> Self {
        Self {
            inner: vec![],
            len: 0,
            radix: 29,
        }
    }

    //.......................................................................//

    pub fn with_radix(mut self, radix: usize) -> Self {
        self.radix = radix;

        self
    }

    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.inner = Vec::with_capacity(capacity);

        for _ in 0..capacity {
            self.inner.push(LinkedList::new());
        }

        self
    }

    //.......................................................................//

    fn hash(&self, val: T) -> usize {
        let mut res = 0;

        for token in Vec::<u8>::from(val) {
            res = (res * self.radix + usize::from(token)) % self.inner.len();
        }

        res
    }

    //.......................................................................//

    pub fn insert(&mut self, val: T) {
        let hash = self.hash(val.clone());

        if let Some(_) = self.inner[hash].search(&val) {
        } else {
            self.inner[hash].push(val);
            self.len += 1;
        }
    }

    //.......................................................................//

    pub fn remove(&mut self, val: T) {
        let hash = self.hash(val.clone());

        if let Some(index) = self.inner[hash].search(&val) {
            self.inner[hash].delete(index);
            self.len -= 1;
        }
    }

    //.......................................................................//

    pub fn contains(&self, val: T) -> bool {
        let hash = self.hash(val.clone());

        self.inner[hash].search(&val).is_some()
    }

    //.......................................................................//

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    //.......................................................................//

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.inner.len()
    }

    //.......................................................................//

    pub fn items(&self) -> Vec<T> {
        self.inner
            .iter()
            .flat_map(|e| e.iter().map(|e| e.clone()))
            .collect()
    }
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basics() {
        let mut table = HashSet::new().with_capacity(500);

        assert!(table.is_empty());

        table.insert("a");

        assert!(!table.is_empty());
        assert!(table.contains("a"));
        assert!(!table.contains("b"));
        assert!(!table.contains("c"));
        assert_eq!(table.len(), 1);

        table.remove("a");

        assert!(table.is_empty());
        assert!(!table.contains("a"));
        assert_eq!(table.len(), 0);

        table.insert("a");

        assert!(!table.is_empty());
        assert!(table.contains("a"));
        assert_eq!(table.len(), 1);

        table.insert("b");

        assert!(!table.is_empty());
        assert!(table.contains("a"));
        assert!(table.contains("b"));
        assert_eq!(table.len(), 2);

        table.insert("c");

        assert!(!table.is_empty());
        assert!(table.contains("a"));
        assert!(table.contains("b"));
        assert!(table.contains("c"));
        assert_eq!(table.len(), 3);

        table.remove("a");

        assert!(!table.is_empty());
        assert!(!table.contains("a"));
        assert!(table.contains("b"));
        assert!(table.contains("c"));
        assert_eq!(table.len(), 2);

        table.remove("b");

        assert!(!table.is_empty());
        assert!(!table.contains("a"));
        assert!(!table.contains("b"));
        assert!(table.contains("c"));
        assert_eq!(table.len(), 1);

        table.insert("c");

        assert!(!table.is_empty());
        assert!(!table.contains("a"));
        assert!(!table.contains("b"));
        assert!(table.contains("c"));
        assert_eq!(table.len(), 1);

        table.remove("c");

        assert!(table.is_empty());
        assert!(!table.contains("a"));
        assert!(!table.contains("b"));
        assert!(!table.contains("c"));

        table.remove("c");

        assert!(table.is_empty());
        assert!(!table.contains("a"));
        assert!(!table.contains("b"));
        assert!(!table.contains("c"));
    }
}

///////////////////////////////////////////////////////////////////////////////
