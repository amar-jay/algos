/*!
  You are given the heads of two sorted linked lists list1 and list2.

Merge the two lists in a one sorted list. The list should be made by splicing together the nodes of the first two lists.

Return the head of the merged linked list.

@examples
Input: list1 = [1,2,4], list2 = [1,3,4]
Output: [1,1,2,3,4,4]
  */
use crate::lessons::linked_list::*;

pub struct Solution {}

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list1 = list1;
        let mut list2 = list2;
        let head = &mut list1;
        while list2.is_some() {
            if list1.is_none() || list1.as_ref()?.val > list2.as_ref()?.val {
                std::mem::swap(head, &mut list2);
            }

            head = &mut head.as_mut()?.next;
        }

        list1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lessons::linked_list::to_list;
    #[test]
    fn test() {
        assert_eq!(Solution::merge_two_lists(None, None), None);
        assert_eq!(Solution::merge_two_lists(None, Some(Box::new(ListNode::new(0)))), Some(Box::new(ListNode::new(0))));
        assert_eq!(Solution::merge_two_lists(Some(Box::new(ListNode::new(0))), None), Some(Box::new(ListNode::new(0))));

    }

    #[test]
    fn test2() {
        let list1 = vec![1, 2, 4];
        let list2 = vec![1, 3, 4];
        let sol = vec![1, 1, 2, 3, 4, 4];
        let list1 = to_list(&list1);
        let list2 = to_list(&list2);
        let sol = to_list(&sol);
        assert_eq!(Solution::merge_two_lists(
            list1,
            list2,
        ), sol);
    }
}

