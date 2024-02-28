use super::solution::*;

///////////////////////////////////////////////////////////////////////////////

#[test]
fn base() {
    // [Source] https://rust-unofficial.github.io/too-many-lists/first-final.html

    let mut list = LinkedList::new();

    assert_eq!(list.pop(), None);

    list.push(30);
    list.push(0);

    assert_eq!(list.pop(), Some(0));
    assert_eq!(list.pop(), Some(30));

    list.push(12);
    list.push(-91);
    list.push(-90);

    assert_eq!(list.pop(), Some(-90));
    assert_eq!(list.pop(), Some(-91));
    assert_eq!(list.pop(), Some(12));
    assert_eq!(list.pop(), None);
}

#[test]
fn peek() {
    // [Source] https://rust-unofficial.github.io/too-many-lists/second-peek.html

    let mut list = LinkedList::new();

    assert_eq!(list.peek(), None);
    assert_eq!(list.peek_mut(), None);

    list.push(30);

    assert_eq!(list.peek(), Some(&30));
    assert_eq!(list.peek_mut(), Some(&mut 30));

    list.peek_mut().map(|item| *item = 15);

    assert_eq!(list.peek(), Some(&15));
    assert_eq!(list.pop(), Some(15));
}

#[test]
fn into_iter() {
    // [Source] https://rust-unofficial.github.io/too-many-lists/second-into-iter.html

    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
}

#[test]
fn iter() {
    // [Source] https://rust-unofficial.github.io/too-many-lists/second-iter.html
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
}

#[test]
fn iter_mut() {
    // [Source] https://rust-unofficial.github.io/too-many-lists/second-iter-mut.html
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.iter_mut();
    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next(), Some(&mut 2));
    assert_eq!(iter.next(), Some(&mut 1));
}

//-----------------------------------------------------------------------//

#[test]
fn read() {
    let mut list = LinkedList::new();

    list.push(4);
    list.push(3);
    list.push(2);
    list.push(1);

    assert_eq!(list.read(0), Some(&1));
    assert_eq!(list.read(1), Some(&2));
    assert_eq!(list.read(2), Some(&3));
    assert_eq!(list.read(3), Some(&4));
    assert_eq!(list.read(4), None);
    assert_eq!(list.read(5), None);
}

#[test]
fn insert() {
    let mut list = LinkedList::new();

    list.push(5);
    list.push(4);
    list.push(3);
    list.push(1);

    list.insert(1, 2);
    list.insert(0, 0);
    assert!(list.insert(7, 10).is_none());

    assert_eq!(list.read(0), Some(&0));
    assert_eq!(list.read(1), Some(&1));
    assert_eq!(list.read(2), Some(&2));
    assert_eq!(list.read(3), Some(&3));
    assert_eq!(list.read(4), Some(&4));
    assert_eq!(list.read(5), Some(&5));
    assert!(list.read(6).is_none());
    assert!(list.read(7).is_none());
}

#[test]
fn delete() {
    let mut list = LinkedList::new();

    list.push(4);
    list.push(3);
    list.push(2);
    list.push(1);

    list.delete(3);

    assert!(list.delete(8).is_none());

    list.delete(0);

    assert_eq!(list.read(0), Some(&2));
    assert_eq!(list.read(1), Some(&3));
    assert_eq!(list.read(2), None);
    assert_eq!(list.read(3), None);
    assert_eq!(list.read(4), None);
}

#[test]
fn search() {
    let mut list = LinkedList::new();

    list.push(4);
    list.push(3);
    list.push(2);
    list.push(1);

    assert_eq!(list.search(&3), Some(2));
    assert_eq!(list.search(&1), Some(0));

    assert_eq!(list.search(&8), None);
}

#[test]
fn sort() {
    let mut list = LinkedList::new();

    list.push(2); // 6
    list.push(6); // 5
    list.push(4); // 4
    list.push(1); // 3
    list.push(5); // 2
    list.push(3); // 1

    println!("{:?}", list);

    list.sort();

    println!("{:?}", list);

    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), Some(2));
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(4));
    assert_eq!(list.pop(), Some(5));
    assert_eq!(list.pop(), Some(6));
    assert_eq!(list.pop(), None);

    let mut list = LinkedList::new();
    list.push(0);
    list.pop();
    list.sort();
    assert_eq!(list.pop(), None);
}

#[test]
fn miri_testing_from_book_ref() {
    let mut list = LinkedList::new();

    list.push(1);
    list.push(2);
    list.push(3);

    assert!(list.pop() == Some(3));
    list.push(4);
    assert!(list.pop() == Some(4));
    list.push(5);

    assert!(list.peek() == Some(&5));
    list.push(6);
    list.peek_mut().map(|x| *x *= 10);
    assert!(list.peek() == Some(&60));
    assert!(list.pop() == Some(60));

    for elem in list.iter_mut() {
        *elem *= 100;
    }

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&500));
    assert_eq!(iter.next(), Some(&200));
    assert_eq!(iter.next(), Some(&100));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);

    assert!(list.pop() == Some(500));
    list.peek_mut().map(|x| *x *= 10);
    assert!(list.peek() == Some(&2000));
    list.push(7);
}

///////////////////////////////////////////////////////////////////////////////
