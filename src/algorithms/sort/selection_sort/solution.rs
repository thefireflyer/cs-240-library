///////////////////////////////////////////////////////////////////////////////

// --- Quick visualization
//
// [5  9  1  0  6  4]
//  ?
//  m  ^
//  m  .  ^
//  i  .  .  ^
//  i  .  .  m  ^
//  i  .  .  m  .  ^
//  i  .  .  m  .  .
//  v  .  .  v  .  .
// [0  9  1  5  6  4]
//  .  ?
//  .  m  ^
//  .  i  m  ^
//  .  i  m  .  ^
//  .  i  m  .  .  ^
//  .  i  m  .  .  .
//  .  v  v  .  .  .
// [0  1  9  5  6  4]
//  .  .  ?
//  .  .  m  ^
//  .  .  i  m  ^
//  .  .  i  m  .  ^
//  .  .  i  .  .  m
//  .  .  v  .  .  v
// [0  1  4  5  6  9]
//  .  .  .  ?
//  .  .  .  m  ^
//  .  .  .  m  .  ^
//  .  .  .  m  .  .
//  .  .  .  v  .  .
// [0  1  4  5  6  9]
//  .  .  .  .  ?
//  .  .  .  .  m  ^
//  .  .  .  .  m  .
//  .  .  .  .  v  .
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
pub fn selection_sort<T>(arr: &mut [T])
where
    T: Ord + std::fmt::Debug + std::fmt::Display + Clone,
{
    /*
    -- Main algorithm (selection sort)

    For each element in `arr`
        |
        | We want to set the current element to be the smallest element
        | from here to the end of the list.
        | This means, each result is bigger than the last,
        | leaving a sorted list.
        |
        | We'll start by assuming the current element is the smallest.
        |
        | We'll iterate from the current index to the end of the array
        |
            |
            | If we find an element smaller than our current smallest,
            | we'll go and update our record.
            |
        |
        | Once we've found the smallest element remaining in the array,
        | we'll use Rust's `array.swap` function to efficiently switch
        | the current element and the smallest element.
        |
        | Again, this always leaves the smallest element in the current
        | spot and leaves a sorted list at the very end.
        |

    That's it! It's a pretty short algorithm.

    */

    // Iterate over each element
    for index in 0..arr.len() {
        // Start by assuming we're already the smallest element
        let mut local_min = index;

        // Iterate over the rightward elements.
        // (skip the one we initialized local min to)
        for local_index in index + 1..arr.len() {
            // Check if we found a new local minimum
            if arr[local_index] < arr[local_min] {
                // If so, update local min
                local_min = local_index;
            }
        }

        // We just swap the smallest and the biggest
        arr.swap(index, local_min);
    }
}

///////////////////////////////////////////////////////////////////////////////

// Time complexity variables:
//   n = the length of `arr`
//
fn _selection_sort<T>(arr: &mut [T])
where
    T: Ord + std::fmt::Debug + std::fmt::Display + Clone,
{
    // iterates n times
    for index in 0..arr.len() {
        // constant time assign
        let mut local_min = index;

        // iterates (n-`index`) times
        for local_index in index + 1..arr.len() {
            // constant time lookup and check
            if arr[local_index] < arr[local_min] {
                // constant time assign
                local_min = local_index;
            }
        }

        // constant time swap
        arr.swap(index, local_min);

        // slowest fragment iterates (n-`index`) times
    }
    // slowest fragment iterates n*(n-`index`) times

    // n*(n-`index`) is more than O(n) so we'll step up to O(n^2) which
    // is only slightly slower than n*(n-`index`)

    // best case is still O(n^2) because the inner loop always runs.

    // average case is therefore, also, O(n^2)
}
// total O(n^2) time complexity

///////////////////////////////////////////////////////////////////////////////
