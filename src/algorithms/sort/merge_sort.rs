///////////////////////////////////////////////////////////////////////////////

use std::fmt;

///////////////////////////////////////////////////////////////////////////////

/// Sorts the provided vector in ascending order.
///
/// - Inputs
///     | `arr: Vec<T>`
///     | The vector array to sort
///
/// - Outputs
///     | `Vec<T>`
///     | The sorted vector
///
pub fn merge_sort<T: Clone + Ord + fmt::Debug>(arr: Vec<T>) -> Vec<T> {
    /*
    --- Merge sort

        Merge sort works by splitting arrays up into equal parts, recursively
        sorting them, and then merging them back together in order.

        This implementation could be improved by re-using allocated vectors.

    */

    fn inner<T: Clone + Ord + fmt::Debug>(arr: Vec<&T>) -> Vec<&T> {
        // check if we're small enough to already be sorted
        if arr.len() < 2 {
            // if so, return the array (its already sorted)
            arr.to_vec()
        } else {
            // find the middle point
            let middle = arr.len() / 2;

            // divide the array into two equal parts
            let lower: Vec<&T> = arr[..middle].to_vec();
            let upper: Vec<&T> = arr[middle..].to_vec();

            // recursively sort each separately
            let lower = inner(lower);
            let upper = inner(upper);

            // create a temp vector for merging
            let mut res = vec![];

            // initialize temp lower index
            let mut i = 0;

            // loop over all items in the right half
            for item in upper {
                // since we know both halves are already sorted,
                // we'll run through all values of the left half
                // less than the current item in the right half
                // before moving to the next item in the right.
                // We avoid duplicate work by tracking our progress
                // in `i`
                while i < lower.len() && item > lower[i] {
                    // move next smallest item onto our result vector
                    res.push(lower[i]);
                    // update left half progress
                    i += 1;
                }
                // move next smallest item onto our result vector
                res.push(item);
            }

            // move the remaining values in the left half over to
            // the result vector (we already know they're in order)
            while i < lower.len() {
                res.push(lower[i]);
                i += 1;
            }

            // return the result vector
            res
        }
    }

    // syntax nightmare to turn Vec<T> into Vec<&T> and back again
    // probably can be refactored away, but I didn't have time
    inner(arr.iter().map(|i| i).collect())
        .iter()
        .map(|i| i.to_owned().to_owned())
        .collect()
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {

    use super::*;

    fn helper(cases: Vec<Vec<i32>>) {
        for case in cases {
            let real = case.clone();
            let mut expected = case.clone();

            let real = merge_sort(real);

            expected.sort();

            assert_eq!(real, expected);
        }
    }

    #[test]
    fn special_cases() {
        helper(vec![vec![], vec![1]])
    }

    #[test]
    fn random_cases() {
        helper(vec![
            vec![1, 3, 2],
            vec![2, 3, 1],
            vec![3, 3, 3],
            vec![3, 3, 2],
            vec![593, 52, 0, 40104, 20, 19, 2, 30, 8],
            vec![5, 23, 6, 8, 9, 0, 2],
        ])
    }

    #[test]
    fn sorted_cases() {
        helper(vec![
            vec![1, 2],
            vec![1, 2, 3],
            vec![0, 2, 5, 6, 8, 9, 23],
            vec![-503, 1, 203, 585, 900],
        ]);
    }

    #[test]
    fn reverse_sorted_cases() {
        helper(vec![
            vec![2, 1],
            vec![3, 2, 1],
            vec![5, 4, 3, 2, 1, 0, -40],
            vec![23, 9, 8, 6, 5, 2, 0],
        ]);
    }

    #[test]
    fn test_big_sorted() {
        let big_number = (2 as i32).pow(20);
        let mut arr: Vec<i32> = Vec::with_capacity(big_number as usize);
        for i in 0..big_number {
            arr.push(i);
        }

        helper(vec![arr]);
    }

    #[test]
    fn test_big_rev_sorted() {
        let big_number = (2 as i32).pow(25);
        let mut arr: Vec<i32> = Vec::with_capacity(big_number as usize);
        for i in big_number..0 {
            arr.push(i);
        }

        helper(vec![arr]);
    }
}

///////////////////////////////////////////////////////////////////////////////
