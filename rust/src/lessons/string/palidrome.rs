#[allow(dead_code)]
pub fn is_palidrome(text: &str) -> bool {
    let mut chars = text.chars();
    while let (Some(s1), Some(s2)) = (chars.next(), chars.next_back()) {
        if s1 != s2 {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::is_palidrome;

    #[test]
    fn test_simple() {
        assert_eq!(is_palidrome("Manan"), false);
        assert_eq!(is_palidrome("manam"), true);
    }

    #[test]
    fn test_sentence_reverse() {
        assert_eq!(is_palidrome("step on no pets"), true);
    }
}

