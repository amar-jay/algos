/*!
Given a string s consisting of words and spaces, return the length of the last word in the string.

A word is a maximal 
substring
 consisting of non-space characters only.
*/

pub fn length_of_last_word(s: &str) -> i32 {
    if let Some(x) = s.trim().split(' ').last() {
        return x.len() as i32;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simple() {
        assert_eq!(length_of_last_word("Hello World"), 5);
        assert_eq!(length_of_last_word("luffy is still joyboy"), 6);
    }
    #[test]
    fn test_with_spaces() {
        assert_eq!(length_of_last_word("   fly me   to   the moon  "), 4);
    }
}
