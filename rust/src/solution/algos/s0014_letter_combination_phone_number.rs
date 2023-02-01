
#[allow(unused)]
pub struct Solution {}
impl Solution {
    fn combs(c: u32) -> Vec<char> {
        match c {
            2 => vec!['a', 'b', 'c'],
            3 => vec!['d', 'e', 'f'],
            4 => vec!['g', 'h', 'i'],
            5 => vec!['j', 'k', 'l'],
            6 => vec!['m', 'n', 'o'],
            7 => vec!['p', 'q', 'r', 's'],
            8 => vec!['t', 'u', 'v'],
            9 => vec!['w', 'x', 'y', 'z'],
            _ => vec![],
        }
    }
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut out = vec![];
        for c in digits.chars() {
            let c = c.to_digit(10).unwrap();
            let combs = Self::combs(c);
            if out.is_empty() {
                out = combs.iter().map(|c| c.to_string()).collect();
            } else {
                let mut out2 = vec![];
                for s in out {
                    for c in &combs {
                        out2.push(format!("{}{}", s, c));
                    }
                }
                out = out2;
            }
        }
        out
        
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    
//    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    #[test]
    fn test_simple() {
        let inp = "23".to_string();
        let expected = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"];
        let out = Solution::letter_combinations(inp);
        assert_eq!(out, expected, "expected {:?} to be {:?}", out, expected);
    }
    
    #[test]
    fn test_zeros() {
        let inp = "2".to_string();
        let expected = vec!["a", "b", "c"];
        let out = Solution::letter_combinations(inp);
        assert_eq!(out, expected, "expected {:?} to be {:?}", out, expected);
    }
}
