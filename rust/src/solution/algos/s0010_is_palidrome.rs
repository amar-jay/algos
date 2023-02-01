/*!
Given an integer x, return true if x is a 
palindrome
, and false otherwise.
*/
#[allow(unused)]
pub struct Solution {}
impl Solution {
    #[allow(unused)]
    pub fn is_palindrome(x: i32) -> bool {
        let x = x.to_string();
        let n = x.len();
        let x = x.chars().collect::<Vec<char>>();
        let head = x[..n/2].iter();
        let tail = x[n/2..].iter().rev();        
        head.zip(tail).all(|(a, b)| a == b)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    
//    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    #[test]
    fn test_simple() {
        let inp = 121;
        let expected = true;
        let out = Solution::is_palindrome(inp);
        assert_eq!(out, expected, "expected {:?} to be {:?}", out, expected);

        let inp = 4000129;
        let expected = false;
        let out = Solution::is_palindrome(inp);
        assert_eq!(out, expected, "expected {:?} to be {:?}", out, expected);
    }
    
    #[test]
    fn test_complex() {
        let inp = 12344321;
        let expected = true;
        let out = Solution::is_palindrome(inp);
        assert_eq!(out, expected, "expected {:?} to be {:?}", out, expected);

        let inp = 1234321;
        let expected = true;
        let out = Solution::is_palindrome(inp);
        assert_eq!(out, expected, "expected {:?} to be {:?}", out, expected);

        let inp = 12345321;
        let expected = false;
        let out = Solution::is_palindrome(inp);
        assert_eq!(out, expected, "expected {:?} to be {:?}", out, expected);
    }

    #[test]
    fn test_negative() {
        let inp = -42;
        let expected = false;
        let out = Solution::is_palindrome(inp);
        assert_eq!(out, expected, "expected {:?} to be {:?}", out, expected);

        let inp = -9128347;
        let expected = false;
        let out = Solution::is_palindrome(inp);
        assert_eq!(out, expected, "expected {:?} to be {:?}", out, expected);
    }
}
