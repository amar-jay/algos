
/*!
Given an input string s and a pattern p, implement regular expression matching with support for '.' and '*' where:

'.' Matches any single character.​​​​
'*' Matches zero or more of the preceding element.
The matching should cover the entire input string (not partial).

 

Example 1:

Input: s = "aa", p = "a"
Output: false
Explanation: "a" does not match the entire string "aa".
Example 2:

Input: s = "aa", p = "a*"
Output: true
Explanation: '*' means zero or more of the preceding element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".
Example 3:

Input: s = "ab", p = ".*"
Output: true
Explanation: ".*" means "zero or more (*) of any character (.)".
 

Constraints:

1 <= s.length <= 20
1 <= p.length <= 30
s contains only lowercase English letters.
p contains only lowercase English letters, '.', and '*'.
It is guaranteed for each appearance of the character '*', there will be a previous valid character to match.
*/
#[allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
        dp[0][0] = true;
        for i in 0..=s.len() {
            for j in 1..=p.len() {
                if p[j - 1] == b'*' {
                    dp[i][j] = dp[i][j - 2] || (i > 0 && dp[i - 1][j] && (s[i - 1] == p[j - 2] || p[j - 2] == b'.'));
                } else {
                    dp[i][j] = i > 0 && dp[i - 1][j - 1] && (s[i - 1] == p[j - 1] || p[j - 1] == b'.');
                }
            }
        }
        dp[s.len()][p.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    
//    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    #[test]
    fn test_simple() {
        let inp = "aa".to_string();
        let regex = "a*".to_string();
        let expected = true;
        let out = Solution::is_match(inp, regex);
        assert_eq!(out, expected, "expected {:?} to be {:?}", out, expected);

        let inp = "ab".to_string();
        let regex = "a*".to_string();
        let expected = false;
        let out = Solution::is_match(inp, regex);
        assert_eq!(out, expected, "expected {:?} to be {:?}", out, expected);
    }
    
    #[test]
    fn test_complex() {
        let inp = "ab".to_string();
        let regex = ".*".to_string();
        let expected = true;
        let out = Solution::is_match(inp, regex);
        assert_eq!(out, expected, "expected {:?} to be {:?}", out, expected);
    }
}
