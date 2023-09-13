#[allow(unused)]
pub fn reverse(text: &str) -> String {
    text.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::reverse;

    #[test]
    fn test_simple_reverse() {
        assert_eq!(reverse("Manan"), "nanaM")
    }

    #[test]
    fn test_sentence_reverse() {
        assert_eq!(reverse("step on no pets"), "step on no pets");
    }

}
