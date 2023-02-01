/*!
Given an array nums of n integers, return an array of all the unique quadruplets [nums[a], nums[b], nums[c], nums[d]] such that:

0 <= a, b, c, d < n
a, b, c, and d are distinct.
nums[a] + nums[b] + nums[c] + nums[d] == target
You may return the answer in any order.
*/
#[allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn four_sums(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut nums = nums;
        nums.sort();
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                let mut k = j + 1;
                let mut l = nums.len() - 1;
                while k < l {
                    let sum = nums[i] + nums[j] + nums[k] + nums[l];
                    if sum == target {
                        res.push(vec![nums[i], nums[j], nums[k], nums[l]]);
                        k += 1;
                        l -= 1;
                    } else if sum < target {
                        k += 1;
                    } else {
                        l -= 1;
                    }
                }
            }
        }
        res.dedup();
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    
//    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    #[test]
    fn test_simple() {
        let inp = vec![1,0,-1,0,-2,2];
        let target = 0;
        let expected = vec![vec![-2,-1,1,2], vec![-2,0,0,2], vec![-1,0,0,1]];
        let out = Solution::four_sums(inp, target);
        assert_eq!(out, expected, "expected {:?} to be {:?}", out, expected);
    }
    
    #[test]
    fn test_another() {
        let inp = vec![2,2,2,2,2];
        let target = 8;
        let expected = vec![vec![2,2,2,2]];
        let out = Solution::four_sums(inp, target);
        assert_eq!(out, expected, "expected {:?} to be {:?}", out, expected);
    }
}
