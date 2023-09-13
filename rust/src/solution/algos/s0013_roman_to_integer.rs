
#[allow(unused)]
use crate::solution::algos::Solution;
impl Solution {
    fn convert(x: char) -> i32 {
        match x {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    }
    #[allow(unused)]
    pub fn roman_to_int(s: String) -> i32 {
        let mut res = 0;
        let mut prev = 0;
        for c in s.chars().rev() {
            let cur = Solution::convert(c);
            if cur < prev {
                res -= cur;
            } else {
                res += cur;
            }
            prev = cur;
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
        let inp = "III".to_string();
        let expected = 3;
        let out = Solution::roman_to_int(inp);
        assert_eq!(out, expected, "expected {:?} to be {:?}", out, expected);
    }
    
    #[test]
    fn test_zeros() {
        let inp = "LVIII".to_string();
        let expected = 58;
        let out = Solution::roman_to_int(inp);
        assert_eq!(out, expected, "expected {:?} to be {:?}", out, expected);
    }
}
