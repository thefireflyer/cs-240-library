///////////////////////////////////////////////////////////////////////////////

/*

    Heavily based on the implementations in [this](https://rust-unofficial.github.io/too-many-lists/index.html) book.

    The book and source code is licensed under the MIT license (https://github.com/rust-unofficial/too-many-lists).

    Most of the layout, data types, base functions and unit tests for base
    functions are essentially the same since I preferred their implementation
    to my original one. However, all of the functions actually specified in the
    assignment are my own.

    Due to the sheer number of functions, I'm only going to explain the ones
    needed to complete the assignment. I will also go quite light on the
    underlying memory management and simpler syntax.

*/

///////////////////////////////////////////////////////////////////////////////

// We need to use raw pointers in this project
use std::ptr;

///////////////////////////////////////////////////////////////////////////////

/*
We're just creating a standard struct with a head parameter.

We'll break the head type out into an alias for readiblity.
*/
#[derive(PartialEq, PartialOrd, Debug)]
pub struct LinkedList<T>
where
    T: Ord + std::fmt::Debug,
{
    head: Cursor<T>,
}

//---------------------------------------------------------------------------//

/*
    Here, we're aliasing a raw pointer.

    Last week I mostly managed to avoid underlying memory management concepts,
    but I think I do need to explain it here.

    Rust has 6 primary ways to talk about data.
    - safe ownership (i.e. when you create an object for the first time)
    - safe, shared, immutable, borrows (this is what &T means)
    - safe, exclusive, mutable, borrows (this is what &mut T means)
    - sort of safe, shared, sort of mutable, borrows with something
        called Rc and RefCell. This was what I originally tried to use.
        It did not go well.
    - unsafe, immutable, raw pointers (denoted with *const T)
    - unsafe, mutable, raw pointer (denoted with *mut T)

    Interestingly, we can actually happily write a singly linked list
    using only safe data types. The issue is sorting. We'll come back to that.

    The reason we're not using the 'sort of safe' code is because it's verbose,
    hard to read, and also isn't easily (or sometimes at all!) contained. That
    is, a function that uses interior mutablity might *have* to expose that
    fact in its API. I wanted to avoid these problems, so we're using unsafe
    code with the help of the book reference.

    All of this means we're going to alias Cursor to a raw pointer of a new
    data type, Node<T>.
*/
type Cursor<T> = *mut Node<T>;

//---------------------------------------------------------------------------//

/*
This bit isn't very interesting, just regular node definition as in any langauge.
*/
#[derive(PartialEq, Debug)]
struct Node<T>
where
    T: Ord + std::fmt::Debug,
{
    data: T,
    next: Cursor<T>,
}

///////////////////////////////////////////////////////////////////////////////

/*
Let's define some methods! (these are denoted with &self or similar)
*/
impl<T> LinkedList<T>
where
    T: Ord + std::fmt::Debug,
{
    pub fn new() -> Self {
        /*
        This is also new!
        Here, we're setting head to a raw null pointer.
        This instead of something like Option<>.
        This also means we're going to have to check if head is null moving
        foward.
        */

        Self {
            head: ptr::null_mut(),
        }
    }

    //-----------------------------------------------------------------------//

    pub fn push(&mut self, data: T) {
        self.head = Box::into_raw(Box::new(Node {
            data,
            next: self.head,
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if !self.head.is_null() {
                let head = Box::from_raw(self.head);
                self.head = head.next;
                Some(head.data)
            } else {
                None
            }
        }
    }

    //-----------------------------------------------------------------------//

    pub fn peek(&self) -> Option<&T> {
        unsafe {
            if !self.head.is_null() {
                Some(&(*self.head).data)
            } else {
                None
            }
        }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe {
            if !self.head.is_null() {
                Some(&mut (*self.head).data)
            } else {
                None
            }
        }
    }

    //-----------------------------------------------------------------------//

    fn get(&self, index: usize) -> Cursor<T> {
        /*
        --- unsafe code!
        It's not all that interesting, Rust just likes to seperate safe and
        unsafe code as much as possible, so you have to explicitly mark unsafe
        code. This ends up meaning that the compiler won't try and statically
        check a variety of safety properties and we can use raw pointers.
        The book talks about this more in depth, but specifically, you can
        deference raw pointers. Writing to pointers is apperently safe on its
        own.
        */
        unsafe {
            /*
            This is a bit weird, we're creating a mutable pointer that points to
            a mutable pointer.

            It basically just means,
            - I want to keep track of a given node,
            - I want to be able to look at a different node,
            - and I also want to be able to change data about whatever node
                I'm looking at.

             */
            let mut cursor = self.head;

            // Iterating until we find our given node or reach the end of the list
            for _ in 0..index {
                if !cursor.is_null() {
                    cursor = (*cursor).next;
                } else {
                    return ptr::null_mut();
                }
            }

            cursor
        }
    }

    //-----------------------------------------------------------------------//

    pub fn read<'a>(&'a self, index: usize) -> Option<&'a T> {
        unsafe {
            /*
            Let's get our given node, then make sure its valid.
            If it so, dereference it (with the weird *cursor syntax)
            and return the node's data.
            Otherwise, return None.
             */
            let cursor = self.get(index);
            if !cursor.is_null() {
                Some(&(*cursor).data)
            } else {
                None
            }
        }
    }

    //-----------------------------------------------------------------------//

    pub fn insert(&mut self, index: usize, value: T) -> Option<()> {
        /*
        Insert is a little more finicky because we need to make sure we aren't
        trying to get the preceding node of nothing.

        We'll just check if we're trying to insert at the head, and handle it
        seperately.
        - We'll create a new node with the given data
        - We'll have the new node's next field point to our current head
        - We'll set our head to the new node
        - Return Some(()) so say everything went okay

        Otherwise,
        - We'll grab the preceding node
        - Check its valid
        - Create a new node with the given data
        - Set the new node's next field to point to the preceding node's
            next field. Essentially sandwich the new node between the next
            in line and previous in line.
        - Set the preceding node's next field to our new node

        We'll return None if we run into an out bounds exception.

        Box is a safe pointer to heap allocated memory. We want the raw
        raw pointer so we use the aptly named into_raw method.

         */
        unsafe {
            if index == 0 {
                self.head = Box::into_raw(Box::new(Node {
                    data: value,
                    next: self.head,
                }));
                Some(())
            } else {
                let cursor = self.get(index - 1);

                if !cursor.is_null() {
                    (*cursor).next = Box::into_raw(Box::new(Node {
                        data: value,
                        next: (*cursor).next,
                    }));
                    Some(())
                } else {
                    None
                }
            }
        }
    }

    pub fn delete(&mut self, index: usize) -> Option<()> {
        // Not much new here
        unsafe {
            // handle deleting head node separately
            if index == 0 {
                if !self.head.is_null() {
                    self.head = (*self.head).next;
                    return Some(());
                } else {
                    // can't delete a non-existent head
                    return None;
                }
            } else {
                // again, getting the preceding element
                let cursor = self.get(index - 1);

                // handling out of bounds exceptions
                if !cursor.is_null() && !(*cursor).next.is_null() {
                    // set our cursor's next to the node after the
                    // target node.
                    (*cursor).next = (*(*cursor).next).next;
                    return Some(());
                } else {
                    return None;
                }
            }
        }
    }

    //-----------------------------------------------------------------------//

    pub fn search(&self, value: &T) -> Option<usize> {
        // just iterate until we find the target
        unsafe {
            let mut ind = 0;
            let mut cursor = self.head;

            while !cursor.is_null() {
                if value == &(*cursor).data {
                    return Some(ind);
                }
                ind += 1;
                cursor = (*cursor).next;
            }

            None
        }
    }

    //-----------------------------------------------------------------------//

    /* ==Sorting==

    We have a problem. Both insertion and selection sort get a lot of their
    speed from being able to quickly read and write the value of a given node,
    at any time, knowing only the index of the node.

    Because of the borrow checker, we can't do that with a safe system.
    We can only have a mutable reference *or* shared references.
    Worse still, we can't backtrack in a singly linked list! So something like
    insertion sort would have to refind nodes again and again even it we *just*
    got them.

    To me, we have 4 options:
    - sacrifice time performance
    - sacrifice memory performance
    - use interior mutabilty
    - use unsafe rust code

    I actually originally started with interior mutablity before reading the
    list book and it got incredibly gnarly.

    With the book's help, we might be able to do it better this time round but
    it book also explicitly recommends against using interior mutablity.

    Let's try unsafe code!

    */
    pub fn sort(&mut self) {
        unsafe {
            /*
            We're going to try to implement selection sort since
            we can't backtrack.

            Let's set up a new cursor.
            */
            let mut cursor = self.head;

            // iterate over every elem in the array
            while !cursor.is_null() {
                /*
                In a langauge like Java, this bit isn't
                very interesting. We're just creating a
                couple tracking variables that we can update. The
                key is that we can also modify the data
                they point to.
                Again, we're using raw mutable pointers to
                do this.
                 */
                let mut local_min = cursor;

                let mut mini_cursor = cursor;

                // the inner loop, looking for smallest item
                while !mini_cursor.is_null() {
                    // just de-referencing in order to compare values
                    if (*mini_cursor).data < (*local_min).data {
                        local_min = mini_cursor;
                    }
                    // we also have to de-reference to get our next node
                    mini_cursor = (*mini_cursor).next;
                }

                // this is sort of interesting, we have to check
                // our values aren't the same otherwise we have
                // issues with our swap function.
                if cursor != local_min {
                    /*
                    the syntax is pretty weird, but this is just saying
                    - have a raw pointer
                    - i want to modify the stuff it points to
                    - i'll de-reference the pointer to get a node
                    - i'll then get the data field of the node
                    - i'll then turn that into a mutable
                        raw pointer
                     */
                    let cval: *mut _ = &mut (*cursor).data;
                    let mval: *mut _ = &mut (*local_min).data;

                    // aptly named,
                    // swaps the values the pointers point to
                    ptr::swap(cval, mval);
                }

                // move rightward in the list.
                cursor = (*cursor).next;
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

impl<T> Drop for LinkedList<T>
where
    T: Ord + std::fmt::Debug,
{
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

///////////////////////////////////////////////////////////////////////////////

pub struct IntoIter<T>(LinkedList<T>)
where
    T: Ord + std::fmt::Debug + Clone;

impl<T> LinkedList<T>
where
    T: Ord + std::fmt::Debug + Clone,
{
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

//---------------------------------------------------------------------------//

impl<T> Iterator for IntoIter<T>
where
    T: Ord + std::fmt::Debug + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

///////////////////////////////////////////////////////////////////////////////

pub struct Iter<'a, T>
where
    T: Ord + std::fmt::Debug + Clone,
{
    next: Option<&'a Node<T>>,
}

impl<T> LinkedList<T>
where
    T: Ord + std::fmt::Debug + Clone,
{
    pub fn iter(&self) -> Iter<T> {
        unsafe {
            Iter {
                next: self.head.as_ref(),
            }
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Ord + std::fmt::Debug + Clone,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.map(|node| {
                self.next = node.next.as_ref();
                &node.data
            })
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

pub struct IterMut<'a, T>
where
    T: Ord + std::fmt::Debug + Clone,
{
    next: Option<&'a mut Node<T>>,
}

impl<T> LinkedList<T>
where
    T: Ord + std::fmt::Debug + Clone,
{
    pub fn iter_mut(&mut self) -> IterMut<T> {
        unsafe {
            IterMut {
                next: self.head.as_mut(),
            }
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T>
where
    T: Ord + std::fmt::Debug + Clone,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.take().map(|node| {
                self.next = node.next.as_mut();
                &mut node.data
            })
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
