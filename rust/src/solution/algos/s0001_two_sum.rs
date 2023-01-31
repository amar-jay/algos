use std::collections::HashMap;
/**
 * [1] Two Sum
 *
 * Given an array of integers, return indices of the two numbers such that they
 * add up to a specific target.
 *
 * You may assume that each input would have exactly one solution, and you may
 * not use the same element twice.
 *
 * Example:
 *
 *
 * Given nums = [2, 7, 11, 15], target = 9,
 *
 * Because nums[0] + nums[1] = 2 + 7 = 9,
 * return [0, 1].
 *
 */
#[allow(unused)]
pub struct Solution {}

impl Solution {
    #[allow(unused)]
    pub fn two_sum(arr: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::with_capacity(arr.len());
        for (idx, num) in arr.iter().enumerate() {
            match map.get(&(target - num)) {
                None => {
                    map.insert(num, idx);
                },
                Some(sub_idx) => return vec![*sub_idx as i32, idx as i32],
            }
        }
            vec![]
}

    #[allow(unused)]
    pub fn aliter(arr: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, x) in arr.iter().enumerate() {
            let i = i as i32;
            if let Some(idx) = arr.iter().position(|&r| r == target-x) {
                return vec![i, idx as i32];
            }
        }
        vec![]
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simple() {
        let nums =  vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
    }
}
