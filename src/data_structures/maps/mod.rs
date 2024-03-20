///////////////////////////////////////////////////////////////////////////////

pub mod avl;
pub mod bst;

///////////////////////////////////////////////////////////////////////////////

pub trait Map {
    type Key;
    type Value;

    fn new() -> Self;
    fn insert(&mut self, key: Self::Key, value: Self::Value) -> bool;
    fn remove(&mut self, key: &Self::Key) -> bool;

    fn contains_key(&self, key: &Self::Key) -> bool;

    fn get(&self, key: &Self::Key) -> Option<&Self::Value>;
    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value>;

    fn keys(&self) -> Vec<&Self::Key>;
    fn values(&self) -> Vec<&Self::Value>;

    fn len(&self) -> usize;
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {

    use std::fmt;

    use tests::bst::BST;

    use self::avl::AVL;

    use super::*;

    #[test]
    fn all() {
        tests(BST::new());
        tests(AVL::new());
    }

    fn tests<T: Map<Key = i32, Value = i32> + fmt::Debug>(mut map: T) {
        assert_eq!(map.len(), 0);
        for i in 0..30 {
            println!("--- {}", i);
            assert!(!map.contains_key(&i));

            println!("{:?}", map);

            for j in 0..i {
                assert!(map.insert(j, j * j));
                println!("{:?}", map);
                assert_eq!(map.len(), (j + 1).try_into().unwrap());
                assert!(map.contains_key(&j));
                assert_eq!(map.get(&j), Some(&(j * j)));
            }

            println!("{:?}", map);

            for j in 0..i {
                assert!(!map.insert(j, j * 2));
                assert_eq!(map.len(), i.try_into().unwrap());
                assert!(map.contains_key(&j));
                assert_eq!(map.get(&j), Some(&(j * 2)));
            }

            println!("{:?}", map);

            for j in 0..i {
                assert!(map.remove(&j));
                println!("{:?}", map);
                assert_eq!(map.len(), (i - j - 1).try_into().unwrap());
                assert!(!map.contains_key(&j));
                assert_eq!(map.get(&j), None);
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
