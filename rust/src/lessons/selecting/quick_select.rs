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
#[allow(unused)]
pub fn quick_select<T>(arr: &[T], k: usize) -> T 
where T:Ord + Copy  {
    arr[0]
}
#[cfg(test)]
mod tests {
 
}
