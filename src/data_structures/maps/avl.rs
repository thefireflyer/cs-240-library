///////////////////////////////////////////////////////////////////////////////

use core::fmt;
use std::{fmt::Debug, marker::PhantomData, ptr::NonNull};

use super::Map;

///////////////////////////////////////////////////////////////////////////////

pub struct AVL<T: Ord, U> {
    root: Cursor<T, U>,

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
    size: usize,
    height: i32,
    skew: i32,
}

///////////////////////////////////////////////////////////////////////////////

impl<T: Ord, U> AVL<T, U> {
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
                        size: 1,
                        height: 1,
                        skew: 0,
                    }))));
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
                            size: 1,
                            height: 1,
                            skew: 0,
                        }))));
                    self.bubble_up(parent);
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
                            size: 1,
                            height: 1,
                            skew: 0,
                        }))));
                    self.bubble_up(parent);

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

    fn subtree_at(&self, cursor: Cursor<T, U>, index: usize) -> Cursor<T, U> {
        unsafe {
            cursor.and_then(|node| {
                let n_left = ((*node.as_ptr()).left)
                    .and_then(|child| Some((*child.as_ptr()).size))
                    .unwrap_or(0);

                if index < n_left {
                    self.subtree_at((*node.as_ptr()).left, index)
                } else if index == n_left {
                    Some(node)
                } else {
                    self.subtree_at((*node.as_ptr()).right, index - n_left - 1)
                }
            })
        }
    }

    //-----------------------------------------------------------------------//

    fn bubble_up(&mut self, cursor: Cursor<T, U>) {
        unsafe {
            if let Some(node) = cursor {
                self.update_props(node);

                if (*node.as_ptr()).skew > 1 || (*node.as_ptr()).skew < -1 {
                    let (right_height, right_skew) = (*node.as_ptr())
                        .right
                        .and_then(|node| Some(((*node.as_ptr()).height, (*node.as_ptr()).skew)))
                        .unwrap_or((0, 0));

                    if right_skew == 1 || right_skew == 0 {
                        self.right_rotate(
                            node,
                            (*node.as_ptr()).right.expect("something went very wrong"),
                        );
                    } else if right_height == -1 {
                        self.right_rotate(
                            node,
                            (*node.as_ptr()).right.expect("something went very wrong"),
                        );
                        self.left_rotate(
                            (*node.as_ptr())
                                .right
                                .expect("something got really messed up"),
                            node,
                        );
                    }
                }

                self.bubble_up((*node.as_ptr()).parent);
            }
        }
    }

    //-----------------------------------------------------------------------//

    fn right_rotate(&mut self, left: NonNull<Node<T, U>>, right: NonNull<Node<T, U>>) {
        unsafe {
            let parent = (*left.as_ptr()).parent;
            let middle_child = (*right.as_ptr()).left;

            if let Some(parent) = parent {
                if (*parent.as_ptr()).left == Some(left) {
                    (*parent.as_ptr()).left = Some(right);
                } else {
                    (*parent.as_ptr()).right = Some(right);
                }
            } else {
                self.root = Some(right);
            }

            (*left.as_ptr()).parent = Some(right);
            (*right.as_ptr()).parent = parent;

            (*left.as_ptr()).right = middle_child;
            (*right.as_ptr()).left = Some(left);

            self.update_props(left);
            self.update_props(right);
        }
    }

    //-----------------------------------------------------------------------//

    fn left_rotate(&mut self, right: NonNull<Node<T, U>>, left: NonNull<Node<T, U>>) {
        unsafe {
            let parent = (*right.as_ptr()).parent;
            let middle_child = (*left.as_ptr()).right;

            if let Some(parent) = parent {
                if (*parent.as_ptr()).left == Some(right) {
                    (*parent.as_ptr()).left = Some(left);
                } else {
                    (*parent.as_ptr()).right = Some(left);
                }
            }

            (*right.as_ptr()).parent = Some(left);
            (*left.as_ptr()).parent = parent;

            (*right.as_ptr()).left = middle_child;
            (*left.as_ptr()).right = Some(right);

            self.update_props(left);
            self.update_props(right);
        }
    }

    //-----------------------------------------------------------------------//

    fn update_props(&mut self, node: NonNull<Node<T, U>>) {
        unsafe {
            (*node.as_ptr()).size = (*node.as_ptr())
                .left
                .and_then(|node| Some((*node.as_ptr()).size))
                .unwrap_or(0)
                + (*node.as_ptr())
                    .right
                    .and_then(|node| Some((*node.as_ptr()).size))
                    .unwrap_or(0)
                + 1;

            let left_height = (*node.as_ptr())
                .left
                .and_then(|child| Some((*child.as_ptr()).height))
                .unwrap_or(0);
            let right_height = (*node.as_ptr())
                .right
                .and_then(|child| Some((*child.as_ptr()).height))
                .unwrap_or(0);

            (*node.as_ptr()).height = 1 + left_height.max(right_height);

            (*node.as_ptr()).skew = right_height - left_height;
        }
    }

    //-----------------------------------------------------------------------//
}

///////////////////////////////////////////////////////////////////////////////

impl<T: Ord, U> Map for AVL<T, U> {
    //-----------------------------------------------------------------------//

    type Key = T;

    type Value = U;

    //-----------------------------------------------------------------------//

    fn new() -> Self {
        AVL {
            root: None,
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

                    self.bubble_up(Some(node));

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
        unsafe {
            self.root
                .and_then(|node| Some((*node.as_ptr()).size))
                .unwrap_or(0)
        }
    }

    //-----------------------------------------------------------------------//
}

///////////////////////////////////////////////////////////////////////////////

impl<T: Ord + fmt::Debug, U: fmt::Debug> Debug for AVL<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            f.debug_struct("BST")
                .field(
                    "root",
                    &self.root.and_then(|node| (Some(&(*node.as_ptr())))),
                )
                .finish()
        }
    }
}

//---------------------------------------------------------------------------//

impl<T: Ord + fmt::Debug, U: fmt::Debug> Debug for Node<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            f.debug_struct("Node")
                // .field("key", &self.key)
                // .field("value", &self.value)
                .field(
                    "left",
                    &self.left.and_then(|node| (Some(&(*node.as_ptr())))),
                )
                .field(
                    "right",
                    &self.right.and_then(|node| (Some(&(*node.as_ptr())))),
                )
                .field("size", &self.size)
                // .field("height", &self.height)
                // .field("skew", &self.skew)
                .finish()
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
