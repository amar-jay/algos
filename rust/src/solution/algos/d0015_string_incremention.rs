/*
* Your job is to write a function which increments a string, to create a new string.

If the string already ends with a number, the number should be incremented by 1.
If the string does not end with a number. the number 1 should be appended to the new string.
Examples:

foo -> foo1

foobar23 -> foobar24

foo0042 -> foo0043

foo9 -> foo10

foo099 -> foo100
*/

use regex::Regex;
// TODO: Ough to fix it still not working
#[allow(unused)]
fn increment_string(s: &str) -> String {
    let re = Regex::new(
        r"(?x)^(?P<word>[a-zA-Z]+)(?P<name>\d*)$"
    ).expect("Wrong regex expression");
    //let x = re.replace_all(s, "$word$name+1");

    let x = re.captures(s).unwrap();
    let (num, word)= (x.name("name").unwrap().as_str(), 
        x.name("word").unwrap().as_str());
    let mut str = String::from(word); 
    if num.is_empty() {
        str.push('1');
        println!("{:#?}", str);
        return str;
    }
    let num = num.trim().parse::<i32>().unwrap() + 1;
     
    str.push_str(&num.to_string());
    
    
    str
}

#[cfg(test)]
mod tests {
    use super::increment_string;

    fn dotest(s: &str, expected: &str) {
        let actual = increment_string(s);
        assert!(actual == expected, 
            "Test failed with s = \"{s}\"\nExpected \"{expected}\"\nBut got \"{actual}\"")
    }
    
 //   #[test]
    #[allow(unused)]
    #[allow(clippy::todo)]
    fn sample_tests() {
        dotest("foo", "foo1");
        dotest("foobar001", "foobar002");
        dotest("foobar1", "foobar2");
        dotest("foobar00", "foobar01");
        dotest("foobar99", "foobar100");
        dotest("foobar099", "foobar100");
        dotest("", "1");
    }
}
