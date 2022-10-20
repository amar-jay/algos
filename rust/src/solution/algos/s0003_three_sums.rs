use std::collections::HashMap;

/**
 * [15] Three Sums
 *
 * Given an array nums of n integers, are there elements a, b, c in nums such that a + b + c = 0?
 * Find all unique triplets in the array which gives the sum of zero.
 *
 * Note:
 *
 * The solution set must not contain duplicate triplets.
 *
 * Example:
 *
 *
 * Given array nums = [-1, 0, 1, 2, -1, -4],
 *
 * A solution set is:
 * [
 *   [-1, 0, 1],
 *   [-1, -1, 2]
 * ]
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/3sum/
// discuss: https://leetcode.com/problems/3sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    #[allow(dead_code)]
    pub fn solution_2(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut combs:HashMap<i32,i32> = HashMap::new();
        let mut nums = nums;
        if nums.len() < 3 {
            return vec![];
        }
        nums.sort();

        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if i==j || combs.get(&(i as i32)).is_some() || combs.get(&(j as i32)).is_some() {
                    continue;
                }
            combs.insert(i as i32, j as i32); 
        }
    }
        let mut ans = vec![];
        for i in combs.keys() {
            let j = combs[&i];
            let k = -i-combs[&i];
            if nums.contains(&k) {
                ans.push(vec![*i, j, k])
            }
        }
        ans
    }

    #[allow(dead_code)]
    pub fn solution_1(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
         if nums.len() < 3 {
            return vec![];
        }
        let mut nums = nums;
        nums.sort();
        nums.dedup();        // find two sums of all
        println!("{:#?}", nums);
        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if nums[i] != nums[j]  && nums[i] != -nums[i]-nums[j] && nums[j] != -nums[i]-nums[j] && i != j && nums.contains(&(-nums[i]-nums[j])) {
                    ans.push(vec![nums[i], nums[j], - nums[i] - nums[j]]);
                }

            }
        }

        ans


    }

    #[allow(dead_code)]
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::solution_2(nums)
    }

}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_15() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
        assert_eq!(
            Solution::three_sum(vec![
                -7, -4, -6, 6, 4, -6, -9, -10, -7, 5, 3, -1, -5, 8, -1, -2, -8, -1, 5, -3, -5, 4,
                2, -5, -4, 4, 7
            ]),
            vec![
                vec![-10, 2, 8],
                vec![-10, 3, 7],
                vec![-10, 4, 6],
                vec![-10, 5, 5],
                vec![-9, 2, 7],
                vec![-9, 3, 6],
                vec![-9, 4, 5],
                vec![-8, 2, 6],
                vec![-8, 3, 5],
                vec![-8, 4, 4],
                vec![-7, -1, 8],
                vec![-7, 2, 5],
                vec![-7, 3, 4],
                vec![-6, -2, 8],
                vec![-6, -1, 7],
                vec![-6, 2, 4],
                vec![-5, -3, 8],
                vec![-5, -2, 7],
                vec![-5, -1, 6],
                vec![-5, 2, 3],
                vec![-4, -4, 8],
                vec![-4, -3, 7],
                vec![-4, -2, 6],
                vec![-4, -1, 5],
                vec![-3, -2, 5],
                vec![-3, -1, 4],
                vec![-2, -1, 3],
                vec![-1, -1, 2]
            ]
        );
        assert_eq!(
            Solution::three_sum(vec![2, 0, -2, -5, -5, -3, 2, -4]),
            vec![vec![-4, 2, 2], vec![-2, 0, 2]]
        );
        let empty_vec: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::three_sum(vec![]), empty_vec);
    }
}
