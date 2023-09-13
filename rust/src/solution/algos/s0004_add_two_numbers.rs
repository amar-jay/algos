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
    #[allow(non_snake_case, dead_code)]
    pub fn addTwoNumbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut ans = Box::new(ListNode::default());
        let list1 = &l1;
        let list2 = &l2;
        let mut tail = &mut ans;
        let mut carry = 0;

        while list1.is_some() || list2.is_some() {
            match (list1, list2) {
                (Some(l1), Some(l2)) => {
                    let sum = l1.val + l2.val + carry;
                    tail.val = sum % 10;
                    carry = sum / 10;
                },
                (Some(l1), None) => {
                    tail.val = l1.val + carry;
                    carry = 0;
                },
                (None, Some(l2)) => {
                    tail.val = l2.val + carry;
                    carry = 0;
                },
                (None, None) => {
                    break;
                },
            }
            tail.next = Some(Box::new(ListNode::default()));
            tail = tail.next.as_mut().unwrap();
        }

        Some(ans)
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


#[cfg(test)]
mod tests {
    // use crate::lessons::linked_list::*;

    // use super::*;

    // #[test]
    // fn test_simple() {
    //     assert_eq!(
    //         Solution::addTwoNumbers(
    //             to_list(&vec![5, 6, 4]),
    //             to_list(&vec![2, 4, 3])),
    //         to_list(&vec![7, 0, 8]),
    //         "{:#?} not equal to {:#?}",
    //         Solution::addTwoNumbers(
    //             to_list(&vec![2, 4, 3]),
    //             to_list(&vec![5, 6, 4])),
    //          to_list(&vec![7, 0, 8])
    //     );
    // }
    // #[test]
    // fn test_carry() {
    //     assert_eq!(
    //         Solution::addTwoNumbers(
    //             to_list(&vec![9, 9, 9, 9]),
    //             to_list(&vec![9, 9, 9, 9, 9, 9])),
    //         to_list(&vec![8, 9, 9, 9, 0, 0, 1])
    //     );
    // }
    // #[test]
    // fn test_single_digit() {
    //     assert_eq!(
    //         Solution::addTwoNumbers(
    //             to_list(&vec![1]),
    //             to_list(&vec![2])
    //         ),
    //         to_list(&vec![0])
    //     );
    // }


}
