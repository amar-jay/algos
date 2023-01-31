/**
*Given an integer array nums of length n and an integer target,
find three integers in nums such that the sum is closest to target.
Return the sum of the three integers.
You may assume that each input would have exactly one solution.
*
*/
pub struct Solution {}
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        nums.windows(3)
            .map(|w| w.iter().sum())
            .fold((0, i32::MAX), |(s, d), x| {
                let d2 = ((x - target) as i32).abs();
                if d2 < d {
                    (x, d2)
                } else {
                    (s, d)
                }
            })
            .0
        
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;
    
//    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    #[test]
    fn test_simple() {
        let inp = vec![-1, 2, 1, -4];
        let target = 1;
        let expected = 2;
        let out = Solution::three_sum_closest(inp, target);
        assert_eq!(out, expected, "expected {:?} to be {:?}", out, expected);
    }
    
    #[test]
    fn test_zeros() {
        let inp = vec![0, 0, 0];
        let target = 1;
        let expected = 0;
        let out = Solution::three_sum_closest(inp, target);
        assert_eq!(out, expected, "expected {:?} to be {:?}", out, expected);
    }
}
