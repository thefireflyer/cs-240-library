///////////////////////////////////////////////////////////////////////////////

use core::fmt;
use std::{fmt::Debug, marker::PhantomData, ptr::NonNull};

use super::Map;

///////////////////////////////////////////////////////////////////////////////

pub struct BST<T: Ord, U> {
    root: Cursor<T, U>,
    size: usize,

    _ghost: PhantomData<T>,
}

//---------------------------------------------------------------------------//

type Cursor<T, U> = Option<NonNull<Node<T, U>>>;

//---------------------------------------------------------------------------//

struct Node<T: Ord, U> {
    key: T,
    value: U,
    left: Cursor<T, U>,
    right: Cursor<T, U>,
    parent: Cursor<T, U>,
}

///////////////////////////////////////////////////////////////////////////////

impl<T: Ord, U> BST<T, U> {
    //-----------------------------------------------------------------------//

    fn get_node(&self, key: &T, cursor: Cursor<T, U>) -> Cursor<T, U> {
        unsafe {
            cursor.and_then(|curr| {
                let data = &(*curr.as_ptr()).key;

                if data == key {
                    cursor
                } else if data > key {
                    self.get_node(key, (*curr.as_ptr()).left)
                } else {
                    self.get_node(key, (*curr.as_ptr()).right)
                }
            })
        }
    }

    //-----------------------------------------------------------------------//

    fn get_min_node(&self, mut cursor: Cursor<T, U>) -> Cursor<T, U> {
        unsafe {
            while let Some(curr) = cursor {
                if (*curr.as_ptr()).left.is_some() {
                    cursor = (*curr.as_ptr()).left;
                } else {
                    return cursor;
                }
            }
            None
        }
    }

    //-----------------------------------------------------------------------//

    fn insert_rec(&mut self, cursor: Cursor<T, U>, key: T, value: U, parent: Cursor<T, U>) -> bool {
        unsafe {
            match (cursor, parent) {
                (None, None) => {
                    self.root = Some(NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                        key,
                        value,
                        left: None,
                        right: None,
                        parent: None,
                    }))));
                    self.size += 1;
                    true
                }
                (None, Some(par)) if (*par.as_ptr()).key > key => {
                    (*par.as_ptr()).left =
                        Some(NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                            key,
                            value,
                            left: None,
                            right: None,
                            parent,
                        }))));
                    self.size += 1;
                    true
                }
                (None, Some(par)) => {
                    (*par.as_ptr()).right =
                        Some(NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                            key,
                            value,
                            left: None,
                            right: None,
                            parent,
                        }))));
                    self.size += 1;
                    true
                }
                (Some(curr), _) if (*curr.as_ptr()).key == key => {
                    (*curr.as_ptr()).value = value;
                    false
                }
                (Some(curr), _) if (*curr.as_ptr()).key > key => {
                    self.insert_rec((*curr.as_ptr()).left, key, value, cursor)
                }
                (Some(curr), _) => self.insert_rec((*curr.as_ptr()).right, key, value, cursor),
            }
        }
    }

    //-----------------------------------------------------------------------//

    fn in_order<F, R>(&self, cursor: Cursor<T, U>, func: F, res: &mut Vec<R>)
    where
        F: Fn(NonNull<Node<T, U>>) -> R + Copy,
    {
        unsafe {
            if let Some(curr) = cursor {
                self.in_order((*curr.as_ptr()).left, func, res);
                res.push(func(curr));
                self.in_order((*curr.as_ptr()).right, func, res);
            }
        }
    }

    //-----------------------------------------------------------------------//
}

///////////////////////////////////////////////////////////////////////////////

impl<T: Ord, U> Map for BST<T, U> {
    //-----------------------------------------------------------------------//

    type Key = T;

    type Value = U;

    //-----------------------------------------------------------------------//

    fn new() -> Self {
        BST {
            root: None,
            size: 0,
            _ghost: PhantomData,
        }
    }

    //-----------------------------------------------------------------------//

    fn insert(&mut self, key: Self::Key, value: Self::Value) -> bool {
        self.insert_rec(self.root, key, value, None)
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        unsafe {
            self.get_node(key, self.root)
                .and_then(|node| {
                    self.size -= 1;

                    let replacement = match ((*node.as_ptr()).left, (*node.as_ptr()).right) {
                        (None, None) => None,
                        (None, Some(child)) => Some(child),
                        (Some(child), None) => Some(child),
                        (Some(_), Some(right)) => self.get_min_node(Some(right)),
                    };

                    (*node.as_ptr()).left.and_then(|child| {
                        (*child.as_ptr()).parent = (*node.as_ptr()).parent;
                        Some(())
                    });

                    (*node.as_ptr()).right.and_then(|child| {
                        (*child.as_ptr()).parent = (*node.as_ptr()).parent;
                        Some(())
                    });

                    if let Some(par) = (*node.as_ptr()).parent {
                        if (*par.as_ptr()).left == Some(node) {
                            (*par.as_ptr()).left = replacement;
                        } else {
                            (*par.as_ptr()).right = replacement;
                        }
                    } else {
                        self.root = replacement;
                    }

                    Some(())
                })
                .is_some()
        }
    }

    //-----------------------------------------------------------------------//

    fn contains_key(&self, key: &Self::Key) -> bool {
        self.get_node(key, self.root).is_some()
    }

    //-----------------------------------------------------------------------//

    fn get(&self, key: &Self::Key) -> Option<&Self::Value> {
        unsafe {
            self.get_node(key, self.root)
                .and_then(|node| Some(&(*node.as_ptr()).value))
        }
    }

    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value> {
        unsafe {
            self.get_node(key, self.root)
                .and_then(|node| Some(&mut (*node.as_ptr()).value))
        }
    }

    //-----------------------------------------------------------------------//

    fn keys(&self) -> Vec<&Self::Key> {
        unsafe {
            let mut res = vec![];

            self.in_order(self.root, |node| &(*(node.as_ptr())).key, &mut res);

            res
        }
    }

    fn values(&self) -> Vec<&Self::Value> {
        unsafe {
            let mut res = vec![];

            self.in_order(self.root, |node| &(*(node.as_ptr())).value, &mut res);

            res
        }
    }

    //-----------------------------------------------------------------------//

    fn len(&self) -> usize {
        self.size
    }

    //-----------------------------------------------------------------------//
}

///////////////////////////////////////////////////////////////////////////////

impl<T: Ord + fmt::Debug, U: fmt::Debug> Debug for BST<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BST")
            .field("root", &self.root)
            .field("size", &self.size)
            .finish()
    }
}

//---------------------------------------------------------------------------//

impl<T: Ord + fmt::Debug, U: fmt::Debug> Debug for Node<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("key", &self.key)
            .field("value", &self.value)
            .field("left", &self.left)
            .field("right", &self.right)
            .finish()
    }
}

///////////////////////////////////////////////////////////////////////////////
