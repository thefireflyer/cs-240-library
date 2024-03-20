///////////////////////////////////////////////////////////////////////////////

/*

Very heavily based off [7] (MIT source code)

*/

///////////////////////////////////////////////////////////////////////////////

use std::{fmt::Debug, marker::PhantomData, process::id, ptr::NonNull};

///////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, PartialOrd)]
pub struct LinkedList<T>
where
    T: Ord,
{
    front: Cursor<T>,
    back: Cursor<T>,
    len: usize,

    _ghost: PhantomData<T>,
}

//---------------------------------------------------------------------------//

type Cursor<T> = Option<NonNull<Node<T>>>;

//---------------------------------------------------------------------------//

#[derive(PartialEq, Debug)]
struct Node<T>
where
    T: Ord,
{
    data: T,
    front: Cursor<T>,
    back: Cursor<T>,
}

///////////////////////////////////////////////////////////////////////////////

impl<T> LinkedList<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        Self {
            front: None,
            back: None,
            len: 0,
            _ghost: PhantomData,
        }
    }

    //-----------------------------------------------------------------------//

    pub fn push_front(&mut self, data: T) {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                front: None,
                back: None,
                data,
            })));
            if let Some(old) = self.front {
                (*old.as_ptr()).front = Some(new);
                (*new.as_ptr()).back = Some(old);
            } else {
                self.back = Some(new);
            }
            self.front = Some(new);
            self.len += 1;
        }
    }

    //.......................................................................//

    pub fn push_back(&mut self, data: T) {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                back: None,
                front: None,
                data,
            })));
            if let Some(old) = self.back {
                (*old.as_ptr()).back = Some(new);
                (*new.as_ptr()).front = Some(old);
            } else {
                self.front = Some(new);
            }
            self.back = Some(new);
            self.len += 1;
        }
    }

    //-----------------------------------------------------------------------//

    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            self.front.map(|node| {
                let boxed_node = Box::from_raw(node.as_ptr());
                let result = boxed_node.data;

                self.front = boxed_node.back;
                if let Some(new) = self.front {
                    (*new.as_ptr()).front = None;
                } else {
                    self.back = None;
                }

                self.len -= 1;
                result
            })
        }
    }

    //.......................................................................//

    pub fn pop_back(&mut self) -> Option<T> {
        unsafe {
            self.back.map(|node| {
                let boxed_node = Box::from_raw(node.as_ptr());
                let result = boxed_node.data;

                self.back = boxed_node.front;
                if let Some(new) = self.back {
                    (*new.as_ptr()).back = None;
                } else {
                    self.front = None;
                }

                self.len -= 1;
                result
            })
        }
    }

    //-----------------------------------------------------------------------//

    pub fn front(&self) -> Option<&T> {
        unsafe { self.front.map(|node| &(*node.as_ptr()).data) }
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        unsafe { self.front.map(|node| &mut (*node.as_ptr()).data) }
    }

    //-----------------------------------------------------------------------//

    pub fn back(&self) -> Option<&T> {
        unsafe { self.back.map(|node| &(*node.as_ptr()).data) }
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        unsafe { self.back.map(|node| &mut (*node.as_ptr()).data) }
    }

    //-----------------------------------------------------------------------//

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    //-----------------------------------------------------------------------//

    pub fn clear(&mut self) {
        while self.pop_front().is_some() {}
    }

    //-----------------------------------------------------------------------//

    pub fn iter(&self) -> Iter<T> {
        Iter {
            front: self.front,
            _back: self.back,
            len: self.len,
            _ghost: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            front: self.front,
            _back: self.back,
            len: self.len,
            _ghost: PhantomData,
        }
    }

    //-----------------------------------------------------------------------//

    fn get(&self, index: usize) -> Cursor<T> {
        unsafe {
            let mut cursor;
            let indices;
            let back;

            if index < self.len / 2 {
                cursor = self.front;
                indices = 0..index;
                back = true;
            } else if index < self.len {
                cursor = self.back;
                indices = index + 1..self.len;
                back = false;
            } else {
                return None;
            }

            for _ in indices {
                if let Some(curr) = cursor {
                    if back {
                        cursor = (*curr.as_ptr()).back;
                    } else {
                        cursor = (*curr.as_ptr()).front;
                    }
                } else {
                    return None;
                }
            }

            // let mut cursor = self.front;

            // for _ in 0..index {
            //     if let Some(curr) = cursor {
            //         cursor = (*curr.as_ptr()).back;
            //     } else {
            //         return None;
            //     }
            // }

            cursor
        }
    }

    //-----------------------------------------------------------------------//

    pub fn read<'a>(&'a self, index: usize) -> Option<&'a T> {
        unsafe { self.get(index).map(|node| &(*node.as_ptr()).data) }
    }

    //-----------------------------------------------------------------------//

    pub fn insert(&mut self, index: usize, value: T) -> Option<()> {
        unsafe {
            if index == 0 {
                Some(self.push_front(value))
            } else if index + 1 == self.len {
                Some(self.push_back(value))
            } else {
                self.get(index).and_then(|nex| {
                    println!("okay");
                    (*nex.as_ptr()).front.and_then(|prev| {
                        println!("okay2");
                        println!("{:?}, {:?}", nex, prev);
                        let nexw = Some(nex);
                        let prevw = Some(prev);
                        println!("{:?}, {:?}", nexw, prevw);

                        let node = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                            back: None,
                            front: None,
                            data: value,
                        })));

                        println!("{:?}, {:?}, {:?}", nex, prev, node);

                        let nodew = Some(node);

                        (*prev.as_ptr()).back = nodew;
                        (*nex.as_ptr()).front = nodew;

                        (*node.as_ptr()).back = nexw;
                        (*node.as_ptr()).front = prevw;

                        self.len += 1;

                        Some(())
                    })
                })
            }
        }
    }

    pub fn delete(&mut self, index: usize) -> Option<T> {
        unsafe {
            if index == 0 {
                self.pop_front()
            } else if index + 1 == self.len {
                self.pop_back()
            } else {
                self.get(index).and_then(|tar| {
                    (*tar.as_ptr()).front.and_then(|prev| {
                        let boxed_node = Box::from_raw(tar.as_ptr());
                        let result = boxed_node.data;
                        (*prev.as_ptr()).back = (*tar.as_ptr()).back;

                        Some(result)
                    })
                })
            }
        }
    }

    //-----------------------------------------------------------------------//

    pub fn search(&self, value: T) -> Option<usize> {
        unsafe {
            let mut cursor = self.front;

            for i in 0..self.len {
                if let Some(curr) = cursor {
                    if (*curr.as_ptr()).data == value {
                        return Some(i);
                    } else {
                        cursor = (*curr.as_ptr()).back;
                    }
                }
            }
            None
        }
    }

    //-----------------------------------------------------------------------//
}

///////////////////////////////////////////////////////////////////////////////

impl<T> Drop for LinkedList<T>
where
    T: Ord,
{
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

///////////////////////////////////////////////////////////////////////////////

pub struct Iter<'a, T>
where
    T: Ord,
{
    front: Cursor<T>,
    _back: Cursor<T>,
    len: usize,
    _ghost: PhantomData<&'a T>,
}

//---------------------------------------------------------------------------//

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Ord,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.front.map(|node| unsafe {
                self.len -= 1;
                self.front = (*node.as_ptr()).back;
                &(*node.as_ptr()).data
            })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

///////////////////////////////////////////////////////////////////////////////

pub struct IterMut<'a, T>
where
    T: Ord,
{
    front: Cursor<T>,
    _back: Cursor<T>,
    len: usize,
    _ghost: PhantomData<&'a mut T>,
}

//---------------------------------------------------------------------------//

impl<'a, T> Iterator for IterMut<'a, T>
where
    T: Ord,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.front.map(|node| unsafe {
                self.len -= 1;
                self.front = (*node.as_ptr()).back;
                &mut (*node.as_ptr()).data
            })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

///////////////////////////////////////////////////////////////////////////////

pub struct IntoIter<T>
where
    T: Ord,
{
    list: LinkedList<T>,
}

//---------------------------------------------------------------------------//

impl<T> Iterator for IntoIter<T>
where
    T: Ord,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.len, Some(self.list.len))
    }
}

//---------------------------------------------------------------------------//

impl<T> IntoIterator for LinkedList<T>
where
    T: Ord,
{
    type IntoIter = IntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

//---------------------------------------------------------------------------//

impl<'a, T> IntoIterator for &'a LinkedList<T>
where
    T: Ord,
{
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

///////////////////////////////////////////////////////////////////////////////

impl<T> Debug for LinkedList<T>
where
    T: Ord + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LinkedList")
            .field("front", &(self.front, self.front()))
            .field("back", &(self.back, self.back()))
            .field("len", &self.len)
            .field("_ghost", &self._ghost)
            .finish()
    }
}

///////////////////////////////////////////////////////////////////////////////
