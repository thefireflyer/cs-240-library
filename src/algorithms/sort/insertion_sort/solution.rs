///////////////////////////////////////////////////////////////////////////////

// --- Quick visualization
//
// [5  9  1  0  6  4]
//     ?
//  ^  .
// [5  9  1  0  6  4]
//        ?
//     ^  .
//  ^  1<>9
//  1<>5  .
// [1  5  9  0  6  4]
//           ?
//        ^  .
//     ^  0<>9
//  ^  0<>5  .
//  0<>1  .  .
// [0  1  5  9  6  4]
//              ?
//           ^  .
//        ^  6<>9
// [0  1  5  6  9  4]
//                 ?
//              ^  .
//           ^  4<>9
//        ^  4<>6  .
//     ^  4<>5  .  .
// [0  1  4  5  6  9]

//---------------------------------------------------------------------------//

// --- Public documentation
//
/// Sorts the provided slice in ascending order.
///
/// - Inputs
///     | `arr: &mut [T]`
///     | The slice to sort (mutable)
///
/// - Side-effects
///     | Sorts elements in `arr`
///
//
// --- Function signature
//
// We'll use the generic `<T>` type for each element in the array.
//
// Rust requires us to explicitly mark mutable objects.
// We're definitely changing the input, so we'll mark `arr` as `&mut`
//
// We saw `[T]` slices last week. They're just a way to reference blocks
// of data we only care about for a little while.
//
// `where T: Ord + ...` is how we talk about interfaces in Rust.
// Here, we're saying that `T` should always implement:
// - an interface for ordering (`Ord`), so we can use `<`
// - an interface for pretty printing (`Debug`)
// - and an interface for making objects with the same contents as another
// object (`Clone`)
//
pub fn insertion_sort<T>(arr: &mut [T])
where
    T: Ord + std::fmt::Debug + std::fmt::Display + Clone,
{
    /*
    -- Main algorithm (insertion sort)

    For each element in `arr`
        |
        | Save the value of the current element as `key`
        |
        | Let's just say that previous elements are already sorted.
        | We'll see why as we continue.
        |
        | Since we're assuming previous items are sorted, we can just take the
        | current element and walk backwards to find it's correct spot.
        |
        | Really, we're going to walk backwards and continually swap elements
        | to make room for our current element when we find its correct spot.
        |
        | To do this, we'll create a reverse index, `rev_ind`, and initialize
        | it to one less than our current index. (we already know we're not
        | bigger than ourselves)
        |
        | Then, we'll iterate backwards until we find an element that isn't
        | smaller than us. Because we're already swapping each pair of
        | elements, we're actually already done. Last iteration, we swapped
        | ourselves directly into the right spot.
        |
        | One last note, we'll check to make sure we're not going to walk off
        | the edge of the list with a simple index check.
        |
        | This means each element walks itself into its sorted spot and ends up
        | leaving our whole list sorted.

    That's it!

    */

    // Iterate over every element
    for index in 1..arr.len() {
        // save the current value
        let key = arr[index].clone();

        // initialize our reverse index (skipping ourselves)
        let mut rev_ind = index - 1;

        // make sure we're smaller than the leftward element
        while key < arr[rev_ind] {
            // swap places with the leftward element
            arr.swap(rev_ind + 1, rev_ind);

            // double check we're not going to walk off the edge of the list
            if rev_ind == 0 {
                // if we are, abort! time to move to the next key element
                break;
            }

            // walk leftwards
            rev_ind -= 1;
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

// Time complexity variables:
//   n = the length of `arr`
//
pub fn _insertion_sort<T>(arr: &mut [T])
where
    T: Ord + std::fmt::Debug + std::fmt::Display + Clone,
{
    // iterates n times
    for index in 1..arr.len() {
        // constant time assign
        let key = arr[index].clone();

        // constant time assign
        let mut rev_ind = index - 1;

        // best case, never runs
        // worst case, runs from current to start, so `index` times
        while key < arr[rev_ind] {
            //constant time swap
            arr.swap(rev_ind + 1, rev_ind);

            // constant time check
            if rev_ind == 0 {
                break;
            }
            //constant time decrement
            rev_ind -= 1;
        }

        //slowest fragment iterates `index` times
    }
    // slowest fragment iterates n*`index` times
    // n*`index` is bigger than O(n), so the function steps up to O(n^2)
    // only slightly slower than n*`index`

    // best case, the inner loop never runs, and the slowest fragment is O(n)

    // however, the average time complexity should still be O(n^2) since
    // the inner loop will run and the function is slower than O(n) as
    // a result.
}
// total O(n^2) time complexity

///////////////////////////////////////////////////////////////////////////////
