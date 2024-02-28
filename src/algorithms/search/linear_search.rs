///////////////////////////////////////////////////////////////////////////////

use tailcall::tailcall;

///////////////////////////////////////////////////////////////////////////////

/// Returns the index of `target` in `arr` if it exists
///
/// - Inputs:
///     | `arr: &[T]`
///     | The array to search through
///     |
///     | `target: &T`
///     | The item to search for
///
/// - Returns:
///     | If `target` is present in `arr`
///         | `Some(usize)`
///         | The index of`target` wrapped in `Some()`
///     | Otherwise
///         | `None`
///         | `target` does not exist in `arr`
///
pub fn linear_search_iterative<T>(arr: &[T], target: &T) -> Option<usize>
where
    T: Ord,
{
    // iterate over every element
    for pos in 0..arr.len() {
        // check if the element is the target
        if arr[pos] == *target {
            // if so return our current position
            return Some(pos);
        }
    }
    // iterated through every item and couldn't find `target`...
    // `target` must not exist in `arr`, we'll let the consumer know
    None
}

//---------------------------------------------------------------------------//

/// Returns the index of `target` in `arr` if it exists
///
/// - Inputs:
///     | `arr: &[T]`
///     | The array to search through
///     |
///     | `target: &T`
///     | The item to search for
///
/// - Returns:
///     | If `target` is present in `arr`
///         | `Some(usize)`
///         | The index of`target` wrapped in `Some()`
///     | Otherwise
///         | `None`
///         | `target` does not exist in `arr`
///
pub fn linear_search_recursive<T>(arr: &[T], target: &T) -> Option<usize>
where
    T: Ord,
{
    /*
    To minimize the risk of stack overflow, we'll use a specialized package for
    optimizing our recursive functions. Tailcall is a MIT licensed package that
    provides the tailcall macro #[tailcall]. It uses a method called trampolining.
    (https://en.wikipedia.org/wiki/Tail_call#Through_trampolining)

    All we need to know is that it won't change the behaviour of our function, but
    behind the scenes turns it into iterative code.

    This can be demostrated with the `test_big` test case.
    Try running `cargo test --release`
    Then try commenting out `#[tailcall]` and re-running `cargo test --release`
    Do you get `fatal runtime error: stack overflow`?
    */

    #[tailcall]
    fn inner<T>(arr: &[T], target: &T, index: usize) -> Option<usize>
    where
        T: Ord,
    {
        /*
        - check if our element is valid
            | if it is
                - check if it is our target
                    | if so, return our current index
                    | otherwise, recursively move onto the next item
            | otherwise, we reached the end of the list and couldn't find target
            | let's return None so the consumer knows
         */
        if index < arr.len() && arr[index] == *target {
            Some(index)
        } else if index < arr.len() {
            inner(arr, target, index + 1)
        } else {
            None
        }
    }

    // start search at the first element
    inner(arr, target, 0)
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {

    use std::vec;

    use super::*;

    fn helper<T>(arr: &[T], target: T, expected: Option<usize>)
    where
        T: Ord,
    {
        let lin_res = linear_search_iterative(&arr, &target);
        let rec_res = linear_search_recursive(&arr, &target);

        assert_eq!(lin_res, rec_res);
        assert_eq!(rec_res, expected);
    }

    #[test]
    fn test_empty_list() {
        helper(&vec![], 30, None);
        helper(&vec![], -50, None);
        helper(&vec![], 0, None);
    }

    #[test]
    fn test_nonexistent_target() {
        helper(&vec![1], 0, None);
        helper(&vec![1, 2], -4, None);
        helper(&vec![0, 1, 3], 2, None);
        helper(&vec!["test", "tes", "te", "t"], "not there", None);
    }

    #[test]
    fn test_existent_target() {
        helper(&vec![1], 1, Some(0));
        helper(&vec![1, 2], 1, Some(0));
        helper(&vec![1, 2], 2, Some(1));
        helper(&vec![1, 2, 3], 1, Some(0));
        helper(&vec![1, 2, 3], 2, Some(1));
        helper(&vec![1, 2, 3], 3, Some(2));
        helper(&vec!["test", "tes", "te", "t"], "test", Some(0));
        helper(&vec!["test", "tes", "te", "t"], "tes", Some(1));
        helper(&vec!["test", "tes", "te", "t"], "te", Some(2));
        helper(&vec!["test", "tes", "te", "t"], "t", Some(3));
    }

    // #[test]
    // fn test_big() {
    //     let big_number = (2 as i32).pow(30);
    //     let mut arr: Vec<i32> = Vec::with_capacity(big_number as usize);
    //     for i in 0..big_number {
    //         arr.push(i);
    //     }

    //     helper(&arr, 0, Some(0));
    //     helper(&arr, big_number - 1, Some((big_number - 1) as usize));
    //     helper(&arr, big_number, None);
    //     helper(&arr, 50, Some(50));
    // }
}

///////////////////////////////////////////////////////////////////////////////
