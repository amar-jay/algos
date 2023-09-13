//!# Quick Select
//!
//!## Problem Statement
//! Given an array, find the kth largest/ smallest element in a linear time complexity. 
//!
//! ### Approach
//!- Select a pivot element at random
//!- Apply partitioning as used in quick sort
//!- After partitioning, the pivot will be placed in its sorted location ie. All elements smaller than the pivot will be on its left and greater on its right
//!- If index of sorted pivot is k, then the pivot is our kth element and we return the number
//!- Else, check if 'k' is greater or smaller and choose a new pivot in that range.
//!- Repeat till we get the kth element at kth position
//!
//!### Founder's Name
//!- This algorithm was developed by Tony Hoare and is also called `Hoare's Selection Algorithm`.
//!## Resources
//!- [github.com/TheAlgorithm](https://github.com/TheAlgorithms/Algorithms-Explanation/blob/master/en/Selection%20Algorithms/Quick%20Select.md)
//! 

/**
 * [QuickSelect](https://www.geeksforgeeks.org/quickselect-algorithm/) is an algorithm to find the kth smallest number
 *
 * Notes:
 * -QuickSelect is related to QuickSort, thus has optimal best and average
 * -case (O(n)) but unlikely poor worst case (O(n^2))
 * -This implementation uses randomly selected pivots for better performance
 *
 * @complexity: O(n) (on average )
 * @complexity: O(n^2) (worst case)
 * @flow
 */

fn partition(arr: &mut [i32], low: usize, high: usize) -> usize
where i32:Ord   {
    let pivot = arr[high];
    let mut i = low;
    for j in low..high {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, high);
    i
}
#[allow(unused)]
pub fn quick_select(arr: &mut [i32],n:i32, low: usize, high: usize) -> usize
{
    let mid = partition(arr, low, high);

    if arr[mid] == n {
        return mid;
    } else if arr[mid] > n {
        return quick_select(&mut arr[..mid], n, low, mid - 1);
    } else {
        return quick_select(&mut arr[mid + 1..], n, mid + 1, high);
    }
}
#[cfg(test)]
mod tests {
    use super::quick_select;

    #[test]
    fn test_easy() {
        let mut arr = [3,4, 5, 6, 8, 9];
        const N:usize = 5; // array length-1 
        let ans = quick_select(&mut arr, 5, 0, N);
        assert_eq!(ans, 2);
    }
}
