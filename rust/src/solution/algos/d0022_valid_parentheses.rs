/**
 ## Valid Parantheses

 Write a function that takes a string of parentheses, and determines if the order of the parentheses is valid. The function should return true if the string is valid, and false if it's invalid.
 ### Constraits

 - 0 <= input.length <= 100
 - Do not treat of forms of brackets as Parantheses
 - Input string may not contain any parantheses at all
*/
#[allow(unused)]
fn valid_parentheses(s: &str) -> bool {
    let mut parens = vec![];

    for c in s.split("") {
        if c == "(" { parens.push(c); }
        if c == ")" {
            match parens.pop() {
                Some(_) => (),
                None => return false,
            }
            }
    }
    parens.is_empty()
}

#[cfg(test)]
mod tests {
    use super::valid_parentheses;
    
    #[test]
    fn sample_tests() {
        do_test("()", true);
        do_test("())", false);
        do_test("", true);
        do_test("(}{)", true);
    }
    
    #[test]
    fn no_paranthese() {
        do_test("", true);
        do_test("[]", true);
    }

    #[test]
    fn non_paranthese_input() {
        do_test("[]", true);
        do_test("{}", true);
        do_test("([]", false);
    }

    fn do_test(s: &str, exp: bool) {
        let actual = valid_parentheses(s); 
        assert_eq!(
            actual,
            exp,
            "\nFor the input \"{}\", your result ({}) did not match the expected output ({})",
            s, actual, exp
        );
    }
}
