/**
* if anything in the text isn't a letter, 
* ignore it and don't return it.
*
* "a" = 1, "b" = 2, etc.
*
*/

#[allow(unused)]
fn alphabet_position(text: &str) -> String {
    let text_bytes:String = text.split_whitespace().collect::<String>().to_uppercase();
    let text_bytes = text_bytes.bytes();

    let mut ans = String::from("");

    for i in text_bytes {
        println!("{:#?}", i);
        if !(65..=64 + 26).contains(&i) {
            continue;
        } else {
        ans.push_str((i-64).to_string().as_str());
        }
        ans.push(' ');
    }
    // Code here...
    println!("{:#?}", ans.trim_end());
    ans.trim_end().to_string()
}

#[test]
fn returns_expected() {
    assert_eq!(
        alphabet_position("The sunset sets at twelve o' clock."),
        "20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11".to_string()
    );
    assert_eq!(
        alphabet_position("The narwhal bacons at midnight."),
        "20 8 5 14 1 18 23 8 1 12 2 1 3 15 14 19 1 20 13 9 4 14 9 7 8 20".to_string()
    );
}
