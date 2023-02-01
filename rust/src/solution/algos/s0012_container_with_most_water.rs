/*!
You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).

Find two lines that together with the x-axis form a container, such that the container contains the most water.

Return the maximum amount of water a container can store.

Notice that you may not slant the container.
*/
#[allow(unused)]
pub struct Solution {}
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut i = 0;
        let mut j = height.len() - 1;
        while i < j {
            let h = height[i].min(height[j]);
            let w = j - i;
            max = max.max(h * w as i32);
            if height[i] < height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    
//    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    #[test]
    fn test_complex() {
        let inp = vec![1,8,6,2,5,4,8,3,7];
        let expected = 49;
        let out = Solution::max_area(inp);
        assert_eq!(out, expected, "expected {:?} to be {:?}", out, expected);
    }
    
    #[test]
    fn test_zeros() {
        let inp = vec![1, 1];
        let expected = 1;
        let out = Solution::max_area(inp);
        assert_eq!(out, expected, "expected {:?} to be {:?}", out, expected);
    }
}
