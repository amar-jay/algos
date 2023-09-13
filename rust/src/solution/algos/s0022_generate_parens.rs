/*!
  @see https://leetcode.com/problems/generate-parentheses/
Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.
*/

use crate::solution::algos::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn generate_parentesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();
        let mut s = String::new();
        Solution::generate(&mut result, &mut s, n, n);
        result
    }

    pub fn generate(result: &mut Vec<String>, s: &mut String, left: i32, right: i32) {
        if left == 0 && right == 0 {
            result.push(s.clone());
            return;
        }
        if left > 0 {
            s.push('(');
            Solution::generate(result, s, left - 1, right);
            s.pop();
        }
        if right > left {
            s.push(')');
            Solution::generate(result, s, left, right - 1);
            s.pop();
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_22() {
        assert_eq!(Solution::generate_parentesis(3), vec!["((()))","(()())","(())()","()(())","()()()"]);
        assert_eq!(Solution::generate_parentesis(1), vec!["()"]);
    }
}
