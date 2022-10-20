/**
 * [3] Longest Substring Without Repeating Characters
 *
 * Given a string, find the length of the longest substring without repeating characters.
 *
 * Example:
 *
 * Input: "abcabcbb"
 * Output: 3
 * Explanation: The answer is "abc", with the length of 3.
 *
 */

#[allow(unused)]
pub struct Solution {}

impl Solution {

    #[allow(unused)]
    pub fn length_of_longest_substring(text: String) -> i32 {
        let seq: Vec<char> = text.chars().collect();
        let len = seq.len();
        let (mut start, mut end, mut max) = (0, 0, 0);
        while end < len {
            for i in start..end {
                if seq[end] == seq[i] {
                    start = i + 1;
                    break;
                }
            }

            let curr = end - start + 1;
            if curr > max {
                max = curr
            }
            end += 1
        }
        max as i32
  }
    
}
