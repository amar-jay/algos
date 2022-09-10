/**
*You are given an array(list) strarr of strings and an integer k.
*Your task is to return the first longest string consisting of k
*consecutive strings taken in the array.
*
*/
#[allow(unused)]
fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    let mut strarr = strarr;
    strarr.sort_by_key(|a| a.len());
    let mut stack = vec![];

    if k < 1 || k > strarr.len() {
        return "".to_string();
    }

    for i in 1..=k {
    stack.push(strarr.remove(strarr.len()-1));

    }

    stack.join("")
}

// Aliter
fn longest_consec2(strarr: Vec<&str>, k: usize) -> String {
    if k > strarr.len() || k == 0 || strarr.is_empty() { String::new() } else {
        strarr.windows(k).map(|x| { x.join("") }).rev().max_by_key(String::len).unwrap()
    }
}

// Testing
#[allow(unused)]
fn testing(strarr: Vec<&str>, k: usize, exp: &str) {
//    assert_eq!(&longest_consec(strarr, k), exp);
    assert_eq!(&longest_consec2(strarr, k), exp);
}

#[test]
fn basics_longest_consec() {
    testing(vec!["zone", "abigail", "theta", "form", "libe", "zas"], 2, "abigailtheta");
    testing(vec!["ejjjjmmtthh", "zxxuueeg", "aanlljrrrxx", "dqqqaaabbb", "oocccffuucccjjjkkkjyyyeehh"], 1, 
        "oocccffuucccjjjkkkjyyyeehh");
    testing(vec![], 3, "");
    testing(vec!["it","wkppv","ixoyx", "3452", "zzzzzzzzzzzz"], 3, "ixoyx3452zzzzzzzzzzzz");
    testing(vec!["it","wkppv","ixoyx", "3452", "zzzzzzzzzzzz"], 15, "");
    testing(vec!["it","wkppv","ixoyx", "3452", "zzzzzzzzzzzz"], 0, "");
}
