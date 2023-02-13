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
    #[allow(unused)]
    pub fn four_sums(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut nums = nums;
        nums.sort();
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                let mut k = j + 1;
                let mut l = nums.len() - 1;
                while k < l {
                    if let Some(sum) = nums[i]
                        .checked_add(nums[j])
                        .and_then(|x| x.checked_add(nums[k]))
                        .and_then(|x| x.checked_add(nums[l])) {
                        if sum == target {
                            if res.contains(&vec![nums[i], nums[j], nums[k], nums[l]]) {
                                k += 1;
                                l -= 1;
                                continue;
                            }
                            res.push(vec![nums[i], nums[j], nums[k], nums[l]]);
                            k += 1;
                            l -= 1;
                        } else if sum < target {
                            k += 1;
                        } else {
                            l -= 1;
                        }
                } else {
                        return vec![];
                    }
                }
            }
        }
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
        assert_eq!(out, expected);
    }
    
    #[test]
    fn test_another() {
        let inp = vec![2,2,2,2,2];
        let target = 8;
        let expected:Vec<Vec<i32>> = vec![vec![2,2,2,2]];
        let out = Solution::four_sums(inp, target);
        assert_eq!(out, expected);
    }

    #[test]
    fn test_big_int() {
        let inp = vec![1000000000,1000000000,1000000000,1000000000];
        let target = -294967296;
        let expected:Vec<Vec<i32>> = vec![];
        let out = Solution::four_sums(inp, target);
        assert_eq!(out, expected);
    }

    #[test]
    fn test_complex() {
        let inp = vec![-5,-4,-3,-2,-1,0,0,1,2,3,4,5];
        let target = 0;
        let expected = vec![
            vec![-5,-4,4,5],
            vec![-5,-3,3,5],
            vec![-5,-2,2,5],
            vec![-5,-2,3,4],
            vec![-5,-1,1,5],
            vec![-5,-1,2,4],
            vec![-5,0,0,5],
            vec![-5,0,1,4],
            vec![-5,0,2,3],
            vec![-4,-3,2,5],
            vec![-4,-3,3,4],
            vec![-4,-2,1,5],
            vec![-4,-2,2,4],
            vec![-4,-1,0,5],
            vec![-4,-1,1,4],
            vec![-4,-1,2,3],
            vec![-4,0,0,4],
            vec![-4,0,1,3],
            vec![-3,-2,0,5],
            vec![-3,-2,1,4],
            vec![-3,-2,2,3],
            vec![-3,-1,0,4],
            vec![-3,-1,1,3],
            vec![-3,0,0,3],
            vec![-3,0,1,2],
            vec![-2,-1,0,3],
            vec![-2,-1,1,2],
            vec![-2,0,0,2],
            vec![-1,0,0,1]
        ];

        let out = Solution::four_sums(inp, target);
        assert_eq!(out, expected, "expected {:?} to be {:?}", out, expected);
    }

    //#[test]
    #[allow(unused)]
    fn test_negative() {
        let inp = vec![0,0,0,-1000000000,-1000000000,-1000000000,-1000000000];
        let target = -1000000000;
        let expected = vec![
            vec![-1000000000,0, 0, 0]
        ];
        let out = Solution::four_sums(inp, target);
        assert_eq!(out, expected);
    }
}
