///////////////////////////////////////////////////////////////////////////////

use core::fmt;

///////////////////////////////////////////////////////////////////////////////

/// A binary min-heap
struct BinaryHeap<T>(Vec<T>);

///////////////////////////////////////////////////////////////////////////////

impl<T> BinaryHeap<T>
where
    T: Ord + Clone + Default + fmt::Debug,
{
    //-----------------------------------------------------------------------//

    /// Creates a new empty binary heap
    ///
    /// - Inputs: N/A
    /// - Output: `BinaryHeap<T>`
    ///     - An empty binary heap
    /// - Side-effects: N/A
    /// - Time complexity: O(1)
    pub fn new() -> Self {
        BinaryHeap(vec![T::default()])
    }

    /// Returns a binary heap with the contents of `source`
    ///
    /// - Inputs:
    ///     - `source: &[T]`
    ///         The slice to build from
    ///
    /// - Output: `BinaryHeap<T>`
    ///     - A binary heap with the contents of `source`
    ///
    /// - Side-effects: N/A
    ///
    /// - Time complexity: O(n)
    ///     - `n = source.len()`
    pub fn from_slice(source: &[T]) -> Self {
        BinaryHeap(Self::heapify(source))
    }

    //-----------------------------------------------------------------------//

    /// Returns a binary heap ordered vector with the contents of `source`
    ///
    /// - Inputs:
    ///     - `source: &[T]`
    ///         The slice to build from
    ///
    /// - Output: `Vec<T>`
    ///     - A vector in binary heap order with elements from `source`.
    ///     - The first element is `T::default()` and should be ignored.
    ///
    /// - Side-effects: N/A
    ///
    /// - Time complexity: O(n)
    ///     - `n = source.len()`
    fn heapify(source: &[T]) -> Vec<T> {
        let n = source.len();

        let mut inner = Vec::with_capacity(n + 1);

        inner.push(T::default());

        for i in 0..n {
            inner.push(source[i].clone());
        }

        for i in (1..n / 2 + 1).rev() {
            Self::bubble_down(&mut inner, i);
        }

        println!("{:?} -> {:?}", source, inner);

        inner
    }

    //-----------------------------------------------------------------------//

    /// Helper function
    fn parent_index(index: usize) -> Option<usize> {
        if index == 1 {
            None
        } else {
            Some(index / 2)
        }
    }

    /// Helper function
    fn left_child_index(index: usize) -> usize {
        index * 2
    }

    /// Helper function
    fn right_child_index(index: usize) -> usize {
        index * 2 + 1
    }

    //-----------------------------------------------------------------------//

    /// Fixes the sub-tree at the given index, moving upwards
    ///
    /// - Inputs:
    ///     - `inner: &mut Vec<T>`
    ///         The heap vector to operate on
    ///     
    ///     - `index: usize`
    ///         The index of the misplaced node
    ///
    /// - Output: N/A
    ///
    /// - Side-effects:
    ///     - Moves the node at the given index upwards in the tree until it
    ///     is in correct binary heap order.
    ///
    /// - Time complexity: O(h-i)
    ///     - `h ≈ log(inner.len())`
    ///     - `i = index`
    ///
    fn bubble_up(inner: &mut Vec<T>, index: usize) {
        if let Some(parent) = Self::parent_index(index) {
            if inner[parent] > inner[index] {
                inner.swap(index, parent);
                Self::bubble_up(inner, parent);
            }
        }
    }

    /// Fixes the sub-tree at the given index, moving downward
    ///
    /// - Inputs:
    ///     - `inner: &mut Vec<T>`
    ///         The heap vector to operate on
    ///     
    ///     - `index: usize`
    ///         The index of the misplaced node
    ///
    /// - Output: N/A
    ///
    /// - Side-effects:
    ///     - Moves the node at the given index downwards in the tree until it
    ///     is in correct binary heap order.
    ///
    /// - Time complexity: O(h-i)
    ///     - `h = log(inner.len())`
    ///     - `i = index`
    ///
    fn bubble_down(inner: &mut Vec<T>, index: usize) {
        let left = Self::left_child_index(index);

        let mut min_index = index;

        for i in 0..2 {
            if left + i <= inner.len() - 1 {
                if inner[min_index] > inner[left + i] {
                    min_index = left + i;
                }
            }
        }
        if min_index != index {
            inner.swap(index, min_index);
            Self::bubble_down(inner, min_index);
        }
    }

    //-----------------------------------------------------------------------//

    /// Inserts the given item in the correct spot
    ///
    /// - Inputs:
    ///     - `&mut self`
    ///     - `item: T` The item to insert
    /// - Output: N/A
    /// - Side-effects:
    ///     - Inserts `item` in the heap
    /// - Time complexity: O(log(n))
    ///     - `n = self.len() + 1`
    pub fn insert(&mut self, item: T) {
        self.0.push(item);
        let n = self.len();
        Self::bubble_up(&mut self.0, n);
    }

    /// Removes the given item
    ///
    /// - Inputs:
    ///     - `&mut self`
    ///     - `item: T` The item to remove
    /// - Output: N/A
    /// - Side-effects:
    ///     - Removes `item` in the heap
    /// - Time complexity: O(n)
    ///     - `n = self.len() + 1`
    pub fn remove(&mut self, item: &T) {
        self.search(item, 1).and_then(|i| Some(self.remove_at(i)));
    }

    //-----------------------------------------------------------------------//

    /// Returns the root (smallest item)
    ///
    /// - Inputs:
    ///     - `&self`
    /// - Output: `&T`
    ///     - The smallest item in the heap
    /// - Side-effects: N/A
    /// - Time complexity: O(1)
    pub fn min(&self) -> &T {
        assert!(self.0.len() > 0);

        &self.0[1]
    }

    /// Removes and returns the root (smallest item)
    ///
    /// - Inputs:
    ///     - `&mut self`
    /// - Output: `&T`
    ///     - The smallest item in the heap
    /// - Side-effects: Removes the smallest item
    /// - Time complexity: O(log(n))
    ///     - `n = self.len() + 1`
    pub fn extract_min(&mut self) -> T {
        let size = self.len();
        assert!(size > 0);

        self.0.swap(1, size);
        let min = self.0.remove(size);
        Self::bubble_down(&mut self.0, 1);
        min
    }

    //-----------------------------------------------------------------------//

    /// Returns the index of the given item
    ///
    /// - Inputs:
    ///     - `&self`
    ///     - `item: &T` The item to look for
    ///     - `index: usize` the index to start from
    /// - Output:
    ///     - If `item` is in the heap
    ///         - `Some(usize)` The index of `item`
    ///     - Else
    ///         - `None`
    /// - Side-effects: N/A
    /// - Time complexity: O(n)
    ///     - `n = self.len() + 1`
    fn search(&self, item: &T, index: usize) -> Option<usize> {
        if index >= self.0.len() {
            None
        } else if item == &self.0[index] {
            Some(index)
        } else if item > &self.0[index] {
            let mut res = None;
            for i in 0..2 {
                res = res.or_else(|| self.search(item, Self::left_child_index(index) + i));
            }
            res
        } else {
            None
        }
    }

    /// Removes the item at the given index
    ///
    /// - Inputs:
    ///     - `&mut self`
    ///     - `index: usize` the index of the item to remove
    /// - Output: `T`
    ///     - The item previously at `index`
    /// - Side-effects: Removes the item at `index` and re-orders to maintain
    ///    heap
    /// - Time complexity: O(log(n))
    ///     - `n = self.len() + 1`
    fn remove_at(&mut self, index: usize) -> T {
        let n = self.len();
        self.0.swap(index, n);
        let val = self.0.remove(n);

        Self::bubble_down(&mut self.0, index);

        val
    }

    //-----------------------------------------------------------------------//

    pub fn len(&self) -> usize {
        self.0.len() - 1
    }

    //-----------------------------------------------------------------------//

    /// Returns the contents of the heap as a sorted vector
    ///
    /// - Inputs:
    ///     - `&mut self`
    /// - Output: `Vec<T>`
    ///     - A sorted vector with the contents of `self`
    /// - Side-effects: N/A
    /// - Time complexity: O(n)
    ///     - `n = self.len() + 1`
    pub fn into_sorted_vec(mut self) -> Vec<T> {
        let mut res = Vec::with_capacity(self.len());

        for i in 0..self.len() {
            res.push(self.extract_min());
        }

        res
    }

    //-----------------------------------------------------------------------//
}

///////////////////////////////////////////////////////////////////////////////

/// Sorts the given vector
pub fn heapsort<T>(list: &mut Vec<T>)
where
    T: Ord + Clone + fmt::Debug + Default,
{
    *list = BinaryHeap::from_slice(&list).into_sorted_vec();
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {

    //-----------------------------------------------------------------------//

    use core::time;
    use std::time::{Duration, SystemTime};

    use super::*;

    //-----------------------------------------------------------------------//

    #[test]
    fn basics() {
        let mut heap = BinaryHeap::new();

        assert_eq!(heap.len(), 0);
        heap.insert(5);
        assert_eq!(heap.len(), 1);
        heap.insert(3);
        assert_eq!(heap.len(), 2);
        heap.insert(7);
        assert_eq!(heap.len(), 3);
        heap.insert(6);
        assert_eq!(heap.len(), 4);
        heap.insert(0);
        assert_eq!(heap.len(), 5);
        assert_eq!(heap.extract_min(), 0);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.extract_min(), 3);
        assert_eq!(heap.len(), 3);
        assert_eq!(heap.extract_min(), 5);
        assert_eq!(heap.len(), 2);
        assert_eq!(heap.extract_min(), 6);
        assert_eq!(heap.len(), 1);
        assert_eq!(heap.extract_min(), 7);
        assert_eq!(heap.len(), 0);
        heap.insert(-15);
        assert_eq!(heap.len(), 1);
        heap.insert(3);
        assert_eq!(heap.len(), 2);
        heap.insert(3);
        assert_eq!(heap.len(), 3);
        heap.insert(8);
        assert_eq!(heap.len(), 4);
        heap.insert(-10);
        assert_eq!(heap.len(), 5);
        assert_eq!(heap.extract_min(), -15);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.extract_min(), -10);
        assert_eq!(heap.len(), 3);
        assert_eq!(heap.extract_min(), 3);
        assert_eq!(heap.len(), 2);
        assert_eq!(heap.extract_min(), 3);
        assert_eq!(heap.len(), 1);
        assert_eq!(heap.extract_min(), 8);
        assert_eq!(heap.len(), 0);
    }

    //-----------------------------------------------------------------------//

    #[test]
    fn general() {
        for i in 0..250 {
            fn test(mut list: Vec<usize>, i: usize) {
                println!("> Test {}", i);
                println!("> ---- {:?}", list);

                let mut heap = BinaryHeap::from_slice(&list);

                list.sort();

                for j in 0..i {
                    assert_eq!(heap.len(), i - j);
                    assert_eq!(heap.min(), &list[j]);
                    assert_eq!(heap.extract_min(), list[j]);
                }

                let mut heap = BinaryHeap::from_slice(&list);

                for j in 0..i {
                    assert_eq!(heap.len(), i - j);
                    assert_eq!(heap.min(), &list[j]);
                    heap.remove(&list[j]);
                }

                let mut heap = BinaryHeap::from_slice(&list);

                for j in 0..i {
                    assert_eq!(heap.len(), i - j);
                    assert_eq!(heap.min(), &list[0]);
                    heap.remove(&list[i - 1 - j]);
                }

                assert_eq!(BinaryHeap::from_slice(&list).into_sorted_vec(), list);
            }

            let list: Vec<usize> = (0..i).rev().collect();
            test(list, i);
            let list: Vec<usize> = (0..i).collect();
            test(list, i);
        }
    }

    //-----------------------------------------------------------------------//

    #[test]
    fn sorting() {
        for i in 0..1000 {
            let mut arr = Vec::with_capacity(i as usize);
            for i in 0..i {
                arr.push(
                    SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_nanos()
                        % 300,
                );
            }

            let mut real = arr.clone();
            let mut expected = arr;

            heapsort(&mut real);
            expected.sort();

            assert_eq!(real, expected);
        }
    }

    //-----------------------------------------------------------------------//
}

///////////////////////////////////////////////////////////////////////////////
