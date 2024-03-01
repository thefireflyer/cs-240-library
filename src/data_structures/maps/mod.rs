///////////////////////////////////////////////////////////////////////////////

pub mod avl;
pub mod bst;

///////////////////////////////////////////////////////////////////////////////

pub trait Map {
    type Key;
    type Value;

    fn insert(&mut self, key: Self::Key, value: Self::Value) -> bool;
    fn remove(&mut self, key: Self::Key) -> bool;

    fn contains_key(&self, key: Self::Key) -> bool;

    fn get(&self, key: Self::Key) -> Self::Value;
    fn get_mut(&mut self, key: Self::Key) -> &mut Self::Value;

    fn keys(&self) -> Vec<Self::Key>;
    fn values(&self) -> Vec<Self::Value>;

    fn len(&self) -> usize;
}

///////////////////////////////////////////////////////////////////////////////
