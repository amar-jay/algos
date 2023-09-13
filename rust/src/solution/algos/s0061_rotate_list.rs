//!
//! Given the head of a linked list, rotate the list to the right by k places.
//! [k=2] 1 -> 2 -> 3 -> 4 -> 5 ==> 4 -> 5 -> 1 -> 2 -> 3
//! [k=4] 0 -> 1 -> 2 ==> 2 -> 0 -> 1
//! ## constaints
//! - The number of nodes in the list is in the range [0, 500].
//! - -100 <= Node.val <= 100
//! 
use crate::lessons::linked_list::ListNode;
use crate::solution::algos::Solution;

impl Solution {

    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
		todo!()
    }
}

#[cfg(test)]
mod tests {
	use super::Solution;

}