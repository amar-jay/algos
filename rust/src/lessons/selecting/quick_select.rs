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


#[allow(unused)]
pub fn quick_select<T:Ord>(arr: &[T], k: usize) -> T {
    arr[0]
}
#[cfg(test)]
mod tests {
 
}
