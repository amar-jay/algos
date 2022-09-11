/**
* Assume "#" is like a backspace in string. This means that string "a#bc#d" actually is "bd"

Your task is to process a string with "#" symbols.

Examples
"abc#d##c"      ==>  "ac"
"abc##d######"  ==>  ""
"#######"       ==>  ""
""              ==>  ""
FUNDAMENTALSSTRINGS
*
*/
#[allow(unused)]
fn solution(s: &str) -> String {
    // your code here
    let mut ans = String::from("");
    for x in s.chars() {
        if x == '#' { 
            if !ans.is_empty() {ans.pop().unwrap();}
        } else {
            ans.push(x);
        } 
    }
    ans
   
}

#[allow(unused)]
fn aliter(s: &str) -> String {
    s.chars().fold(String::new(), |mut acc, c| if c == '#' {acc.pop(); acc} else {acc.push(c); acc})
}

#[allow(unused)]
fn clean_string(s: &str) -> String {
    aliter(s)
   // solution(s);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(clean_string("abc#d##c"), "ac");
        assert_eq!(clean_string("abc####d##c#"), "");
        assert_eq!(clean_string("#######"), "");
        assert_eq!(clean_string(""), "");
    }
}
