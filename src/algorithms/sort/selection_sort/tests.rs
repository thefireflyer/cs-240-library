use crate::algorithms::sort::shared_test_cases::*;

use super::solution::selection_sort;

///////////////////////////////////////////////////////////////////////////////

fn helper(cases: Vec<Vec<i32>>) {
    for case in cases {
        let mut real = case.clone();
        let mut expected = case.clone();

        expected.sort();

        selection_sort(&mut real);

        assert_eq!(real, expected);
    }
}

///////////////////////////////////////////////////////////////////////////////

#[test]
fn special_cases() {
    helper(shared_special_cases())
}

//---------------------------------------------------------------------------//

#[test]
fn random_cases() {
    helper(shared_random_cases())
}

//---------------------------------------------------------------------------//

#[test]
fn sorted_cases() {
    helper(shared_sorted_cases());
}

//---------------------------------------------------------------------------//

#[test]
fn reverse_sorted_cases() {
    helper(shared_reverse_sorted_cases());
}

///////////////////////////////////////////////////////////////////////////////
