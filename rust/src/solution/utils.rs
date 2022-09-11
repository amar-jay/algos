use std::collections::HashMap;

#[allow(unused)]
pub fn from_morse_code() -> HashMap<String, String>{
    let mut map:HashMap<String, String> = HashMap::new();

    let codes = to_morse_code();

    for key in codes.keys() {
        let val = &codes[key];
        map.insert(val.to_string(), key.to_string());
    }

    map
}

#[allow(unused)]
pub fn to_morse_code() -> HashMap<String, String> {
    let mut morse = HashMap::new();
    morse.insert("a".to_string(), ".-".to_string());
    morse.insert("b".to_string(), "-...".to_string());
    morse.insert("c".to_string(),  "-.-".to_string());
    morse.insert("d".to_string(),  "-..".to_string());
    morse.insert("e".to_string(),    ".".to_string());
    morse.insert("f".to_string(), "..-.".to_string());
    morse.insert("g".to_string(),  "--.".to_string());
    morse.insert("h".to_string(), "....".to_string());
    morse.insert("i".to_string(),   "..".to_string());
    morse.insert("j".to_string(), ".---".to_string());
    morse.insert("k".to_string(),   "-.".to_string());
    morse.insert("l".to_string(), ".-..".to_string());
    morse.insert("m".to_string(),   "--".to_string());
    morse.insert("n".to_string(),   "-.".to_string());
    morse.insert("o".to_string(),  "---".to_string());
    morse.insert("p".to_string(), ".--.".to_string());
    morse.insert("q".to_string(), "--.-".to_string());
    morse.insert("r".to_string(), ".-.".to_string());
    morse.insert("s".to_string(),  "...".to_string());
    morse.insert("t".to_string(),   "-".to_string());
    morse.insert("u".to_string(),  "..-".to_string());
    morse.insert("v".to_string(), "...-".to_string());
    morse.insert("w".to_string(),  ".--".to_string());
    morse.insert("x".to_string(), "-..-".to_string());
    morse.insert("y".to_string(), "-.--".to_string());
    morse.insert("z".to_string(), "--..".to_string());
    morse.insert("1".to_string(), ".----".to_string());
    morse.insert("2".to_string(),"..---".to_string());
    morse.insert("3".to_string(), "...--".to_string());
    morse.insert("4".to_string(), "....-".to_string());
    morse.insert("5".to_string(), ".....".to_string());
    morse.insert("6".to_string(), "-....".to_string());
    morse.insert("7".to_string(), "--...".to_string());
    morse.insert("8".to_string(), "---..".to_string());
    morse.insert("9".to_string(), "----.".to_string());
    morse.insert("0".to_string(), "-----".to_string());
    morse
}
