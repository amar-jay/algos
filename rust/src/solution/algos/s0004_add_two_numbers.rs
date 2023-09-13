/**
 * [2] Add Two Numbers
 *
 * You are given two non-empty linked lists representing two non-negative
 * integers. The digits are stored in reverse order and each of their nodes
 * contain a single digit. Add the two numbers and return it as a linked list.
 *
 * You may assume the two numbers do not contain any leading zero, except the
 * number 0 itself.
 *
 * Example:
 *
 *
 * Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
 * Output: 7 -> 0 -> 8
 * Explanation: 342 + 465 = 807.
 *
 */

use crate::lessons::linked_list::ListNode;

use crate::solution::algos::Solution;

// problem: https://leetcode.com/problems/add-two-numbers/
// discuss: https://leetcode.com/problems/add-two-numbers/discuss/?valentPage=1&orderBy=most_votes&query=

impl Solution {
    #[allow(unused)]
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::add(l1, l2, 0)
    }

    pub fn add(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        carry: i32
    ) -> Option<Box<ListNode>> {
        let mut _sum = carry;
        let l1_val = if let Some(node) = l1 {
            _sum+=node.val;
            return node.next;
        } else {None};
        let l2_val = if let Some(node) = l2 {
            _sum+=node.val;
            return node.next;
        } else {None};

        let mut res = ListNode::new(_sum % 10);
        if l1_val.is_some() || l2_val.is_some() || _sum >= 10 {
            res.next = Self::add(l1_val, l2_val, _sum / 10);
        } 

        return Some(Box::new(res))
    }
 
    /*
    pub fn aliter(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        let (mut n1, mut n2) = (0, 0);

        while let Some(l) = l1 {
            n1 = l1.val;

        }

    }

    pub fn solution(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        let (mut n1, mut n2) = (0, 0);

        while let Some(l) = l1 {
            n1 = l1.val;

        }

    }
    */
}

// submission codes end

#[cfg(test)]
mod tests {
    use crate::lessons::linked_list::*;

    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!(
            Solution::add_two_numbers(
                to_list(&vec![5, 6, 4]),
                to_list(&vec![2, 4, 3])),
            to_list(&vec![7, 0, 8]),
            "{:#?} not equal to {:#?}",
            Solution::add_two_numbers(
                to_list(&vec![2, 4, 3]),
                to_list(&vec![5, 6, 4])),
             to_list(&vec![7, 0, 8])
        );
    }
    #[test]
    fn test_carry() {
        assert_eq!(
            Solution::add_two_numbers(
                to_list(&vec![9, 9, 9, 9]),
                to_list(&vec![9, 9, 9, 9, 9, 9])),
            to_list(&vec![8, 9, 9, 9, 0, 0, 1])
        );
    }
    #[test]
    fn test_single_digit() {
        assert_eq!(
            Solution::add_two_numbers(
                to_list(&vec![1]),
                to_list(&vec![2])
            ),
            to_list(&vec![0])
        );
    }


/*
    fn test_2() {
        assert_eq!(
            Solution::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])),
            to_list(vec![7, 0, 8])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![9, 9, 9, 9]), to_list(vec![9, 9, 9, 9, 9, 9])),
            to_list(vec![8, 9, 9, 9, 0, 0, 1])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![0]), to_list(vec![0])),
            to_list(vec![0])
        )
    }
    */
}
