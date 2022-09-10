use regex::Regex;
/**
*
*ATM machines allow 4 or 6 digit PIN codes and PIN codes cannot contain anything but exactly 4 digits or exactly 6 digits.

If the function is passed a valid PIN string, return true, else return false.
*/

#[allow(unused)]
fn validate_pin(pin: &str) -> bool {
    let regex = Regex::new(r"^\d{4}(\d{2})?$").unwrap();

    regex.is_match(pin)
}

#[cfg(test)]
mod tests {
    use super::validate_pin;
    
    #[test]
    fn invalid_length_tests() {
        assert!(!validate_pin("1"));
        assert!(!validate_pin("12"));
        assert!(!validate_pin("123"));
        assert!(!validate_pin("12345"));
        assert!(!validate_pin("1234567"));
        assert!(!validate_pin("-1234"));
        assert!(!validate_pin("1.234"));
        assert!(!validate_pin("-1.234"));
        assert!(!validate_pin("00000000"));
    }
    
    #[test]
    fn non_digit_chars_tests() {
        assert!(!validate_pin("a234"));
        assert!(!validate_pin(".234"));
    }
    
    #[test]
    fn valid_pin_tests() {
        assert!(validate_pin("1234"));
        assert!(validate_pin("0000"));
        assert!(validate_pin("1111"));
        assert!(validate_pin("123456"));
        assert!(validate_pin("098765"));
        assert!(validate_pin("000000"));
        assert!(validate_pin("123456"));
        assert!(validate_pin("090909"));
    }
}
