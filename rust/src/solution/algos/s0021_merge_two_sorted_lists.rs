/*!
  You are given the heads of two sorted linked lists list1 and list2.

Merge the two lists in a one sorted list. The list should be made by splicing together the nodes of the first two lists.

Return the head of the merged linked list.

@examples
Input: list1 = [1,2,4], list2 = [1,3,4]
Output: [1,1,2,3,4,4]
  */
use crate::utils::linked_list::ListNode;

pub struct Solution {}

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ans_head = ListNode::new(0);
        let mut ans = &mut ans_head.next.unwrap();

        let (mut l1_head, mut l1_tail) = (list1, list1);
        let (mut l2_head, mut l2_tail) = (list2, list2);

        let curr = i32::MAX;
        while l1_tail.is_some() && l2_tail.is_some() {
            if l1_tail.unwrap().curr < l2_tail.unwrap().curr {
                ans.curr = l1_tail.unwrap().curr;
                l1_tail = l1_tail.unwrap().next;
            } else {
                ans.curr = l2_tail.unwrap().curr;
                l2_tail = l2_tail.unwrap().next;
            }

        }

        return ans_head.next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::linked_list::to_list;
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

