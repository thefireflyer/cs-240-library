use super::solution::*;

///////////////////////////////////////////////////////////////////////////////

#[test]
fn test_basic_front() {
    // [Source] https://rust-unofficial.github.io/too-many-lists/sixth-final.html

    let mut list = LinkedList::new();

    assert_eq!(list.len(), 0);
    assert_eq!(list.pop_front(), None);
    assert_eq!(list.len(), 0);

    list.push_front(10);
    assert_eq!(list.len(), 1);
    assert_eq!(list.pop_front(), Some(10));
    assert_eq!(list.len(), 0);
    assert_eq!(list.pop_front(), None);
    assert_eq!(list.len(), 0);

    list.push_front(10);
    assert_eq!(list.len(), 1);
    list.push_front(20);
    assert_eq!(list.len(), 2);
    list.push_front(30);
    assert_eq!(list.len(), 3);
    assert_eq!(list.pop_front(), Some(30));
    assert_eq!(list.len(), 2);
    list.push_front(40);
    assert_eq!(list.len(), 3);
    assert_eq!(list.pop_front(), Some(40));
    assert_eq!(list.len(), 2);
    assert_eq!(list.pop_front(), Some(20));
    assert_eq!(list.len(), 1);
    assert_eq!(list.pop_front(), Some(10));
    assert_eq!(list.len(), 0);
    assert_eq!(list.pop_front(), None);
    assert_eq!(list.len(), 0);
    assert_eq!(list.pop_front(), None);
    assert_eq!(list.len(), 0);
}

#[test]
fn test_basic() {
    // [Source] https://rust-unofficial.github.io/too-many-lists/sixth-final.html
    let mut m = LinkedList::new();
    assert_eq!(m.pop_front(), None);
    assert_eq!(m.pop_back(), None);
    assert_eq!(m.pop_front(), None);
    m.push_front(1);
    assert_eq!(m.pop_front(), Some(1));
    m.push_back(2);
    m.push_back(3);
    assert_eq!(m.len(), 2);
    assert_eq!(m.pop_front(), Some(2));
    assert_eq!(m.pop_front(), Some(3));
    assert_eq!(m.len(), 0);
    assert_eq!(m.pop_front(), None);
    m.push_back(1);
    m.push_back(3);
    m.push_back(5);
    m.push_back(7);
    assert_eq!(m.pop_front(), Some(1));

    let mut n = LinkedList::new();
    n.push_front(2);
    n.push_front(3);
    {
        assert_eq!(n.front().unwrap(), &3);
        let x = n.front_mut().unwrap();
        assert_eq!(*x, 3);
        *x = 0;
    }
    {
        assert_eq!(n.back().unwrap(), &2);
        let y = n.back_mut().unwrap();
        assert_eq!(*y, 2);
        *y = 1;
    }
    assert_eq!(n.pop_front(), Some(0));
    assert_eq!(n.pop_front(), Some(1));
}

//-----------------------------------------------------------------------//
/*
#[test]
fn read() {
    let mut list = LinkedList::new();

    list.push_front(4);
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);

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

    list.push_front(5);
    list.push_front(4);
    list.push_front(3);
    list.push_front(1);

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

    list.push_front(4);
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);

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

    list.push_front(4);
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);

    assert_eq!(list.search(3), Some(2));
    assert_eq!(list.search(1), Some(0));

    assert_eq!(list.search(8), None);
}

#[test]
fn sort() {
    let mut list = LinkedList::new();

    list.push_front(2); // 6
    list.push_front(6); // 5
    list.push_front(4); // 4
    list.push_front(1); // 3
    list.push_front(5); // 2
    list.push_front(3); // 1

    println!("{:?}", list);

    list.sort();

    println!("{:?}", list);

    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_front(), Some(4));
    assert_eq!(list.pop_front(), Some(5));
    assert_eq!(list.pop_front(), Some(6));
    assert_eq!(list.pop_front(), None);

    let mut list = LinkedList::new();
    list.push_front(0);
    list.pop_back();
    list.sort();
    assert_eq!(list.pop_front(), None);
}

#[test]
fn miri_testing_from_book_ref() {
    let mut list = LinkedList::new();

    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    assert!(list.pop_front() == Some(3));
    list.push_front(4);
    assert!(list.pop_front() == Some(4));
    list.push_front(5);

    assert!(list.front() == Some(&5));
    list.push_front(6);
    list.front_mut().map(|x| *x *= 10);
    assert!(list.front() == Some(&60));
    assert!(list.pop_front() == Some(60));

    for elem in list.iter_mut() {
        *elem *= 100;
    }

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&500));
    assert_eq!(iter.next(), Some(&200));
    assert_eq!(iter.next(), Some(&100));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);

    assert!(list.pop_front() == Some(500));
    list.front_mut().map(|x| *x *= 10);
    assert!(list.front() == Some(&2000));
    list.push_front(7);
}

///////////////////////////////////////////////////////////////////////////////
*/
