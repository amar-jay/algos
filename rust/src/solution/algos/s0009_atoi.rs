/*!
Implement the myAtoi(string s) function, which converts a string to a 32-bit signed integer (similar to C/C++'s atoi function).

The algorithm for myAtoi(string s) is as follows:

Read in and ignore any leading whitespace.
Check if the next character (if not already at the end of the string) is '-' or '+'. Read this character in if it is either. This determines if the final result is negative or positive respectively. Assume the result is positive if neither is present.
Read in next the characters until the next non-digit character or the end of the input is reached. The rest of the string is ignored.
Convert these digits into an integer (i.e. "123" -> 123, "0032" -> 32). If no digits were read, then the integer is 0. Change the sign as necessary (from step 2).
If the integer is out of the 32-bit signed integer range [-231, 231 - 1], then clamp the integer so that it remains in the range. Specifically, integers less than -231 should be clamped to -231, and integers greater than 231 - 1 should be clamped to 231 - 1.
Return the integer as the final result.
Note:

Only the space character ' ' is considered a whitespace character.
Do not ignore any characters other than the leading whitespace or the rest of the string after the digits.

*/
#![allow(unused)]
pub struct Solution {}
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut str = s.trim().to_string();
        let mut s = str.chars();
        let mut sign = 1;
        if let Some(c) = s.next() {
            if c == '-' {
                sign = -1;
            } else if c == '+' {
                sign = 1;
            } else if c.is_digit(10) {
                str = std::iter::once(c).chain(s).collect::<String>();
                s = str.chars();
            } else {
                return 0;
            }
        } else {
            return 0;
        }
        let mut res:i64 = 0;
        for c in s {
            if c.is_digit(10) {
                res = res * 10 + c.to_digit(10).unwrap() as i64;
                
                if res * sign > i32::MAX as i64 {
                    return i32::MAX;
                } else if res * sign < i32::MIN as i64 {
                    return i32::MIN;
                }
            } else {
                break;
            }
        }
        
        (res * sign) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    
//    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    #[test]
    fn test_positive() {
        let inp = "42".to_string();
        let expected = 42;
        let out = Solution::my_atoi(inp);
        assert_eq!(out, expected, "expected {:?} to be {:?}", out, expected);

        let inp = "4000129".to_string();
        let expected = 4000129;
        let out = Solution::my_atoi(inp);
        assert_eq!(out, expected, "expected {:?} to be {:?}", out, expected);
    }
    
    #[test]
    fn test_with_words() {
        let inp = "4193 with words".to_string();
        let expected = 4193;
        let out = Solution::my_atoi(inp);
        assert_eq!(out, expected, "expected {:?} to be {:?}", out, expected);

        let inp = "I am 20 years old".to_string();
        let expected = 0;
        let out = Solution::my_atoi(inp);
        assert_eq!(out, expected, "expected {:?} to be {:?}", out, expected);
    }

    #[test]
    fn test_negative() {
        let inp = "   -42".to_string();
        let expected = -42;
        let out = Solution::my_atoi(inp);
        assert_eq!(out, expected, "expected {:?} to be {:?}", out, expected);

        let inp = "-91283472332".to_string();
        let expected = -2147483648;
        let out = Solution::my_atoi(inp);
        assert_eq!(out, expected, "expected {:?} to be {:?}", out, expected);
    }
}
