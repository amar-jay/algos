#![allow(dead_code, unused_imports)]

use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::{rc::Rc, cell::RefCell, collections::HashSet};
use std::iter::FromIterator;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
pub enum Solution {}


impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        nums.into_iter().zip(0..).scan(HashMap::<i32, i32>::new(), |map, (num, i)| {
            match map.insert(num, i) {
                Some(j) => Some(i - j <= k),
                None => Some(false),
            }
        }).any(|ok| ok)
    }

    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        nums.len() != HashSet::<i32>::from_iter(nums).len()
    }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32]
                }
            }
        }
        Vec::new()
    }
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut jump = 0;

        for i in 0..nums.len() {
            if i > jump {
                return false
            }
            jump = jump.max(i + nums[i] as usize);
        }

        true
    }

    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut sum = 0;
        nums.iter().for_each(|num| {
            sum += num;
            max = max.max(sum);
            sum = sum.max(0);
        });
        max
    }
        
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list2 = list2;
        let mut list1 = list1;
        let mut head = &mut list1;
        while list2.is_some() {
            if head.is_none() || list2.as_ref()?.val < head.as_ref()?.val {
                std::mem::swap(head, &mut list2);
            }
            head = &mut head.as_mut()?.next;
        }   
        list1
    }


    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let nums: Vec<i32> = nums1.into_iter().chain(nums2.into_iter()).collect();

        match nums.len() % 2 {
            0 => {
                let mid = nums.len() / 2;
                return (nums[mid] + nums[mid - 1]) as f64 / 2.0;
            },
            _ => {
                let mid = nums.len() / 2;
                return nums[mid] as f64;
            }
        }
    }

    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s = s.chars();
        let mut t = t.chars();
        let mut a = s.next();
        let mut b = t.next();

        while a.is_some() && b.is_some() {
            if a.as_ref() == b.as_ref() {
                a = s.next();
            }
            b = t.next();
        }

        a.is_none()
    }

    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut res = digits;
        let mut carry:i32 = 0;
        *res.last_mut().unwrap() += 1;
        for c in res.iter_mut().rev() {
            if carry > 0 {
                *c += carry;
                carry = 0;
            }
            match *c > 9 {
                true => {
                carry = *c / 10;
                *c %= 10;
            }, 
                _ => break
            }
        }
        if carry > 0 {
            res.insert(0, carry);
        }

        res
    }

    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        0
    }
}


mod tests {
        use super::*;
    #[test]
    fn test_contains_duplicate() {
        assert_eq!(Solution::contains_duplicate(vec![1,2,3,1]), true);
        assert_eq!(Solution::contains_duplicate(vec![1,2,3,4]), false);
        assert_eq!(Solution::contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2]), true);
    }

    #[test]
    fn test_contains_nearby_duplicate() {
        // assert_eq!(Solution::contains_nearby_duplicate(vec![1,2,3,1], 3), true);
        assert_eq!(Solution::contains_nearby_duplicate(vec![1,0,1,1], 1), true);
        assert_eq!(Solution::contains_nearby_duplicate(vec![1,2,3,1,2,3], 2), false);
    }

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::two_sum(vec![2,7,11,15], 9), vec![0,1]);
        assert_eq!(Solution::two_sum(vec![3,2,4], 6), vec![1,2]);
        assert_eq!(Solution::two_sum(vec![3,3], 6), vec![0,1]);
    }

    #[test]
    fn test_can_jump() {
        assert_eq!(Solution::can_jump(vec![2,3,1,1,4]), true);
        assert_eq!(Solution::can_jump(vec![3,2,1,0,4]), false);
    }

    #[test]
    fn test_max_sub_array() {
        assert_eq!(Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]), 6);
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
    }

    #[test]
    fn test_merge_two_lists() {

    }

    #[test]
    fn test_is_subsequent() {
        assert_eq!(Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string()), true);
        assert_eq!(Solution::is_subsequence("axc".to_string(), "ahbgdc".to_string()), false);
    }

    #[test]
    fn test_plus_one() {
        assert_eq!(Solution::plus_one(vec![1,2,3]), vec![1,2,4]);
        assert_eq!(Solution::plus_one(vec![4,3,2,1]), vec![4,3,2,2]);
        assert_eq!(Solution::plus_one(vec![0]), vec![1]);
       assert_eq!(Solution::plus_one(vec![9]), vec![1, 0]);
       assert_eq!(Solution::plus_one(vec![8, 9, 9, 9]), vec![9, 0, 0, 0]);
        assert_eq!(Solution::plus_one(vec![9, 9]), vec![1, 0, 0]);
        assert_eq!(Solution::plus_one(vec![9,8,7,6,5,4,3,2,1,0]), vec![9,8,7,6,5,4,3,2,1,1]);
        assert_eq!(Solution::plus_one(vec![7,2,8,5,0,9,1,2,9,5,3,6,6,7,3,2,8,4,3,7,9,5,7,7,4,7,4,9,4,7,0,1,1,1,7,4,0,0,6]), vec![7,2,8,5,0,9,1,2,9,5,3,6,6,7,3,2,8,4,3,7,9,5,7,7,4,7,4,9,4,7,0,1,1,1,7,4,0,0,7])
    }

    #[test]
    fn test_order_of_largest_plus_sign() {
        assert_eq!(Solution::order_of_largest_plus_sign(5, vec![vec![4,2]]), 2);
        assert_eq!(Solution::order_of_largest_plus_sign(1, vec![vec![0,0]]), 0);
    }
}
