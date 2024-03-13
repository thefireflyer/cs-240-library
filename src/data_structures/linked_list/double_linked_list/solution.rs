///////////////////////////////////////////////////////////////////////////////

/*

Very heavily based off [7] (MIT source code)

*/

///////////////////////////////////////////////////////////////////////////////

use std::{marker::PhantomData, ptr::NonNull};

///////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, PartialOrd, Debug)]
pub struct LinkedList<T>
where
    T: Ord + std::fmt::Debug,
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
    T: Ord + std::fmt::Debug,
{
    data: T,
    front: Cursor<T>,
    back: Cursor<T>,
}

///////////////////////////////////////////////////////////////////////////////

impl<T> LinkedList<T>
where
    T: Ord + std::fmt::Debug,
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

    fn _get(&self, _index: usize) -> Cursor<T> {
        todo!()
    }

    //-----------------------------------------------------------------------//

    pub fn read<'a>(&'a self, _index: usize) -> Option<&'a T> {
        todo!()
    }

    //-----------------------------------------------------------------------//

    pub fn insert(&mut self, _index: usize, _value: T) -> Option<()> {
        todo!()
    }

    pub fn delete(&mut self, _index: usize) -> Option<()> {
        todo!()
    }

    //-----------------------------------------------------------------------//

    pub fn search(&self, _value: T) -> Option<usize> {
        todo!()
    }

    //-----------------------------------------------------------------------//

    pub fn sort(&mut self) {
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////////

impl<T> Drop for LinkedList<T>
where
    T: Ord + std::fmt::Debug,
{
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

///////////////////////////////////////////////////////////////////////////////

pub struct Iter<'a, T>
where
    T: Ord + std::fmt::Debug,
{
    front: Cursor<T>,
    _back: Cursor<T>,
    len: usize,
    _ghost: PhantomData<&'a T>,
}

//---------------------------------------------------------------------------//

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Ord + std::fmt::Debug,
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
    T: Ord + std::fmt::Debug,
{
    front: Cursor<T>,
    _back: Cursor<T>,
    len: usize,
    _ghost: PhantomData<&'a mut T>,
}

//---------------------------------------------------------------------------//

impl<'a, T> Iterator for IterMut<'a, T>
where
    T: Ord + std::fmt::Debug + Clone,
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
    T: Ord + std::fmt::Debug + Clone,
{
    list: LinkedList<T>,
}

//---------------------------------------------------------------------------//

impl<T> Iterator for IntoIter<T>
where
    T: Ord + std::fmt::Debug + Clone,
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
    T: Ord + std::fmt::Debug + Clone,
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
    T: Ord + std::fmt::Debug + Clone,
{
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

///////////////////////////////////////////////////////////////////////////////
