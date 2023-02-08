/*!
  You are given the heads of two sorted linked lists list1 and list2.

Merge the two lists in a one sorted list. The list should be made by splicing together the nodes of the first two lists.

Return the head of the merged linked list.

@examples
Input: list1 = [1,2,4], list2 = [1,3,4]
Output: [1,1,2,3,4,4]
  */
use crate::utils::linked_list::ListNode;

#[allow(unused)]
pub struct Solution {}

impl Solution {
    #[allow(unused)]
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut head;
        let mut l1 = list1;
        let mut l2 = list2;
        while l1.is_some() && l2.is_some() {
            let mut node = None;
            if l1.as_ref().unwrap().curr < l2.as_ref().unwrap().curr {
                node = l1.take();
                l1 = node.as_mut().unwrap().next;
            } else {
                node = l2.take();
                l2 = node.as_mut().unwrap().next;
            }
            tail.as_mut().unwrap().next = node;
            tail = &mut tail.as_mut().unwrap().next;
        }
        if l1.is_some() {
            tail.as_mut().unwrap().next = l1;
        }
        if l2.is_some() {
            tail.as_mut().unwrap().next = l2;
        }
        head.unwrap().next
        
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

