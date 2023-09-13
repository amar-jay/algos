#![allow(unused)]
pub fn is_anagram(s: &str, t: &str) -> bool {
    let mut s:Vec<char> = s.to_lowercase().chars().collect();
    let mut t:Vec<char> = t.to_lowercase().chars().collect();
    s.sort();
    t.sort();

    s == t
}


#[cfg(test)]
mod tests {
    use super::is_anagram;
    #[test]
    fn test_anagram() {
        assert!(is_anagram("anagram", "nagaram"));
        assert!(!is_anagram("rat", "car"));
        assert!(is_anagram("abcde", "edcba"));
        assert!(is_anagram("sIlEnT", "LiStEn"));
    }
}

