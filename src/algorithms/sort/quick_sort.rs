///////////////////////////////////////////////////////////////////////////////

use std::fmt;

///////////////////////////////////////////////////////////////////////////////

/// Sorts the provided slice in ascending order.
///
/// - Inputs
///     | `arr: &mut [T]`
///     | The slice to sort (mutable)
///
/// - Side effects
///     | Sorts `arr` in ascending order
///
pub fn quick_sort<T: Ord + fmt::Debug>(arr: &mut [T]) {
    // heavily based on [2]'s implementation
    // see commit `fa58f0d` for quick sort without in place mutation

    /*
    --- Quick sort overview

    We start by checking if our array is empty or only contains 1 element.
    If it is, it is already sorted.
    Otherwise ---
    We start by selecting a pivot. Ideally we want a random pivot to improve
    worst case performance. For simplicity, we're just setting the last
    element of the array to be the pivot.
    Next, we make our problem simpler by splitting up our array.
    One part will consist of elements less than our pivot and the other will
    consist of elements greater than our pivot.
    The details of splitting are described in the `part` function.
    We then recursively sort the two smaller arrays.
    This leaves us with a completely sorted array.

    */

    fn part<T: Ord + fmt::Debug>(arr: &mut [T]) -> usize {
        /*

        Partition method from [2] ---

            We start by picking a pivot. For simplicity, we'll just start
            with the last element.

            In order to avoid allocating a second array, we'll create a
            variable to denote our where we should split our array.
            Elements at indices less than the variable will have values
            less than the pivot. Likewise, the elements to the right of
            the variable will have greater values than the pivot.
            The textbook calls this variable firsthigh, but I feel like
            lower_bound is slightly more descriptive.

            Since we don't know which elements are less than pivot yet,
            we'll initialize lower_bound to zero, denoting an empty array.

            Now, in order to figure out which elements are smaller than
            pivot, we loop over each in our provided array.

            When we find elements smaller than our pivot, we'll simulate
            adding them to a second array by placing them in a designated
            lower area of the provided array. We'll do this by just swapping
            whatever lower_bound currently points to with our current element.
            Now, we need to increment lower_bound one slot to right to show
            our lower area now has one more element.

            Once we've checked each element, we have one more issue.
            The pivot itself is out of place!
            At the start, we didn't know how many items we were going to have
            in the lower area so we couldn't say where the seperation point
            would be. Now we do!
            To place the pivot correctly in between the lower and upper areas,
            we'll just swap it with whatever lower_points to.

            Finally, since we've moved our pivot element to our lower_bounds
            index, and the caller is expecting the index of our pivot, we'll
            return lower_bounds.


        Demo ---

            ^ - current
            * - pivot
            l - lower end

            [3,1,2] --- 3>2
             ^ . .
             l . *
            [3,1,2] --- 1<2 --- swap ^ and l
             . ^ .
             l . *
            [1,3,2] --- 2=2
             . . ^
             . l *

            swap l and *

            [1,2,3]
             . l .

            return l
        */

        // initialize to the last element
        let pivot = arr.len() - 1;

        // initialize to an empty area
        let mut lower_end = 0;

        // look for elements smaller than pivot
        for i in 0..arr.len() {
            if arr[i] < arr[pivot] {
                // move smaller elements into the designated lower area
                arr.swap(i, lower_end);
                // notify ourselves that the lower area is one slot bigger
                lower_end += 1;
            }
        }

        // move the pivot in-between the lower area and upper area
        arr.swap(pivot, lower_end);

        // return the index of our pivot element
        lower_end
    }

    fn inner<T: Ord + fmt::Debug>(arr: &mut [T]) {
        /*
        Sorting method from [2] ---

            This part is pretty simple.
            Just check if we're small enough to already be sorted (size < 2)
            If not, just partition our array and search each side of the
            partition seperately.

        Syntax key ---

            arr[..bound] means we're sectioning off from i=0 to i=bound-1
            arr[bound+1..] means we're sectioning off from i=bound+1 to i=length

            We've seen this before but &mut is a safe mutable reference.
            Basically a point with a couple of safety guarantees and rules.

        */

        if arr.len() > 1 {
            let pivot = part(arr);
            inner(&mut arr[..pivot]);
            inner(&mut arr[pivot + 1..]);
        }
    }

    // run our inner function on the full array
    inner(arr);
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {

    use super::*;

    fn helper(cases: Vec<Vec<i32>>) {
        for case in cases {
            let mut real = case.clone();
            let mut expected = case.clone();

            quick_sort(&mut real);

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
        let big_number = (2 as i32).pow(9);
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

#[cfg(test)]
mod benchmarks {
    // extern crate test;

    // use test::Bencher;

    // fn bench_helper(cases: Vec<Vec<i32>>, b: &mut Bencher) {
    //     for mut case in cases {
    //         b.bench(|_| Ok(quick_sort(&mut case)));
    //     }
    // }

    // #[bench]
    // fn random_cases_b(b: &mut Bencher) {
    //     bench_helper(
    //         vec![
    //             vec![1, 3, 2],
    //             vec![2, 3, 1],
    //             vec![3, 3, 3],
    //             vec![3, 3, 2],
    //             vec![593, 52, 0, 40104, 20, 19, 2, 30, 8],
    //             vec![5, 23, 6, 8, 9, 0, 2],
    //         ],
    //         b,
    //     );
    // }
}

///////////////////////////////////////////////////////////////////////////////
