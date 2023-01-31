
pub struct Solution{}
impl Solution {
    #[allow(unused)]
    pub fn solution_1(strs: Vec<String>) -> String {
        let mut min_len = 0;
        let x = strs.iter()
            .min_by(|x, y| x.len().cmp(&y.len()))
            .map(|x| x.chars()).unwrap();
        for (id, i) in strs.iter().enumerate() {
            for (a, b) in x.clone().zip(i.chars()) {
                if a == b  {
                    min_len = id;
                    break;
                }
            }
        }
        strs[0][0..min_len].to_string()        //ans.collect()
        }
        
    #[allow(unused)]
    pub fn solution_2(strs: Vec<String>) -> String {
        let words:Vec<&str> = strs.iter().map(|c| &**c).collect();
        let shortest = strs.iter().min().unwrap();


        for (i, c) in shortest.chars().enumerate() {
            for word in words.iter() {
                if word.chars().nth(i).unwrap() != c {
                    return word[..i].to_string();
                }
            }
        }
        shortest.to_string()
    }

    #[allow(unused)]
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        Solution::solution_2(strs)
    }
    }
#[cfg(test)]
mod tests {
    use super::*;
    #[test] 
    fn test_simple() {
        let inp = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
        let out = Solution::longest_common_prefix(inp);
        let expected = "fl".to_string();
        assert_eq!(out, expected, "expected {:?} to be {:?}", out, expected);
    }
}
