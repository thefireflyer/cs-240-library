///////////////////////////////////////////////////////////////////////////////

use tailcall::tailcall;

///////////////////////////////////////////////////////////////////////////////

/// Returns index of given target in a sorted list.
///
/// Inputs:
/// - `arr: &[i32]`
///     - The sorted list to check in
///
/// - `target: i32`
///     - The target value to check for
///
/// Output variants:
/// - `Some(index)`
///     - `index` is the position of `target` in `arr`
/// - `None`
///     - `target` is not in `arr`
///
pub fn binary_search_iterative<T>(arr: &[T], target: &T) -> Option<usize>
where
    T: Ord,
{
    // Shortened docs, see week 1 assignment for details and loop invariants

    // Implementation based on: [1;2;3]

    // Initialize our search region to the whole array
    let mut start: usize = 0;
    let mut end: usize = arr.len();

    // Search loop | Find target
    // Run until we've exhausted possible items
    // search_start > search_end would be an invalid search region
    while start < end {
        // Find the midpoint of our search region (size intermediate to avoid integer overflow)
        let size = end - start;
        let midpoint = start + (size / 2);
        // Get the item at the midpoint index
        let item = &arr[midpoint];

        // Check if the item is the one we're looking for
        if item == target {
            // If so, immediately return with the midpoint index
            return Some(midpoint);
        }

        // Check if the item is lower than target
        if item < target {
            // If so, discard the lower half of our search region
            start = midpoint + 1;
        }
        // Check if the item is higher than target
        if item > target {
            // If so, discard the upper half of our search region
            end = midpoint;
        }
    }

    // The target isn't here...
    None
}

//---------------------------------------------------------------------------//

/// Returns index of given target in a sorted list.
///
/// Inputs:
/// - `arr: &[i32]`
///     - The sorted list to check in
///
/// - `target: i32`
///     - The target value to check for
///
/// Output variants:
/// - `Some(index)`
///     - `index` is the position of `target` in `arr`
/// - `None`
///     - `target` is not in `arr`
///
pub fn binary_search_recursive<T>(arr: &[T], target: &T) -> Option<usize>
where
    T: Ord,
{
    /*
    [Inner algorithm]

        |
        | Essentially the same as the iterative approach.
        | We're just breaking up the for loop into a recursive function.
        |
        | Instead of mutable lower and upper bounds, we'll pass them in
        | as function parameters.
        |
        | Inside the function, we find the midpoint and compare it to target.
        |
        | If we need to adjust our search area, we'll call ourselves again,
        | but with adjusted parameters.
        |
        | (We'll need access to `arr` and `target` so we need to pass them in
        | as parameters as well)
        |

    */

    // See linear_search.rs for `#[tailcall]` explanation
    #[tailcall]
    fn inner<T>(arr: &[T], target: &T, lower: usize, upper: usize) -> Option<usize>
    where
        T: Ord,
    {
        // check if we have a valid search area
        if lower < upper {
            // find the midpoint (avoiding integer overflow)
            let size = upper - lower;
            let midpoint = lower + (size / 2);
            // find the midpoint item
            let item = &arr[midpoint];

            // Check if the item is the one we're looking for
            if item == target {
                // If so, immediately return with the midpoint index
                Some(midpoint)
            }
            // Check if the item is lower than target
            else if item < target {
                // If so, discard the lower half of our search region
                // Again, adjusting the parameters to our function call
                // instead of modifying a mutable variable.
                inner(arr, target, midpoint + 1, upper)
            }
            // Item must be higher than target...
            else {
                // If so, discard the upper half of our search region
                // Again, adjusting the parameters to our function call
                // instead of modifying a mutable variable.
                inner(arr, target, lower, midpoint)
            }
        } else {
            // Our search area is empty!
            // Let's let the consumer know we couldn't find `target`
            None
        }
    }

    // start with our search region covering the whole array
    // return the result of our recursive search function
    inner(arr, target, 0, arr.len())
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
        if arr.len() > 0 {
            for ind in 0..arr.len() - 1 {
                assert!(arr[ind] < arr[ind + 1]);
            }
        }

        let lin_res = binary_search_iterative(&arr, &target);
        let rec_res = binary_search_recursive(&arr, &target);

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
        helper(&vec!["t", "te", "tes", "test"], "not there", None);
    }

    #[test]
    fn test_existent_target() {
        helper(&vec![1], 1, Some(0));
        helper(&vec![1, 2], 1, Some(0));
        helper(&vec![1, 2], 2, Some(1));
        helper(&vec![1, 2, 3], 1, Some(0));
        helper(&vec![1, 2, 3], 2, Some(1));
        helper(&vec![1, 2, 3], 3, Some(2));
        helper(&vec!["t", "te", "tes", "test"], "t", Some(0));
        helper(&vec!["t", "te", "tes", "test"], "te", Some(1));
        helper(&vec!["t", "te", "tes", "test"], "tes", Some(2));
        helper(&vec!["t", "te", "tes", "test"], "test", Some(3));
        helper(&vec!["x", "y", "z"], "x", Some(0));
        helper(&vec!["x", "y", "z"], "y", Some(1));
        helper(&vec!["x", "y", "z"], "z", Some(2));
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
