use crate::lessons::linked_list::ListNode;
use crate::solution::algos::Solution;
use std::collections::HashSet;
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		let mut contains = HashSet::new();
		let mut c = head;
		let mut curr = &mut c;

		while curr.is_some() {
			let val = curr.as_ref().unwrap().val;
			if contains.contains(&val) {
				*curr = curr.as_mut().unwrap().next.take();
			} else {
				contains.insert(val);
				curr = &mut curr.as_mut().unwrap().next;
			}
		}
        
		return c;
    }
}

#[cfg(test)]
mod tests {
	use super::*;
use crate::lessons::linked_list::*;
	#[test]
	fn test_simple() {
		assert_eq!(Solution::delete_duplicates(to_list(&vec![1,1,2])), to_list(&vec![1,2]));
		assert_eq!(Solution::delete_duplicates(to_list(&vec![1,1,2,3,3])), to_list(&vec![1,2,3]));
	}

	#[test]
	fn test_empty() {
		assert_eq!(Solution::delete_duplicates(None), None);
	}

	#[test]
	fn test_single() {
		assert_eq!(Solution::delete_duplicates(to_list(&vec![1])), to_list(&vec![1]));
	}

}