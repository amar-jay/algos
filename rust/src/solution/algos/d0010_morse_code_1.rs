//use std::collections::HashMap;

/**
* In this kata you have to write a simple Morse code decoder. While the Morse code 
* is now mostly superseded by voice and digital data communication channels,
* it still has its use in some applications around the world.
The Morse code encodes every character as a sequence of "dots" and "dashes".
For example, the letter A is coded as ·−, letter Q is coded as −−·−, 
and digit 1 is coded as ·−−−−. The Morse code is case-insensitive, traditionally 
capital letters are used. When the message is written in Morse code, a single space
is used to separate the character codes and 3 spaces are used to separate words.
For example, the message HEY JUDE in Morse code is ···· · −·−−   ·−−− ··− −·· ·.

NOTE: Extra spaces before or after the code have no meaning and should be ignored.

In addition to letters, digits and some punctuation, there are some special service codes, the most notorious of those is the international distress signal SOS (that was first issued by Titanic), that is coded as ···−−−···. These special codes are treated as single special characters, and usually are transmitted as separate words.

Your task is to implement a function that would take the morse code as input and return a decoded human-readable string.

For example:
decode_morse(".... . -.--   .--- ..- -.. .")
//should return "HEY JUDE"

NOTE: For coding purposes you have to use ASCII characters . and -, not Unicode characters.

The Morse code table is preloaded for you as a dictionary, feel free to use it:

Coffeescript/C++/Go/JavaScript/Julia/PHP/Python/Ruby/TypeScript: MORSE_CODE['.--']
Elixir: @morse_codes variable (from use MorseCode.Constants). Ignore the unused variable warning for morse_codes because it's no longer used and kept only for old solutions.
Haskell: morseCodes ! ".--" (Codes are in a Map String String)
Java: MorseCode.get(".--")
Kotlin: MorseCode[".--"] ?: "" or MorseCode.getOrDefault(".--", "")
Racket: morse-code (a hash table)
Rust: MORSE_CODE
Swift: MorseCode[".--"] ?? "" or MorseCode[".--", default: ""]
C: provides parallel arrays, i.e. morse[2] == "-.-" for ascii[2] == "C" to the morsecodes, and a corresponding list of ascii symbols

All the test strings would contain valid Morse code, so you may skip checking for errors and exceptions. In C#, tests will fail if the solution code throws an exception, please keep that in mind. This is mostly because otherwise the engine would simply ignore the tests, resulting in a "valid" solution.

Good luck!

After you complete this kata, you may try yourself at Decode the Morse code, advanced.
*
*
*/
use crate::solution::utils;
#[allow(unused)]
fn decode_morse(encoded: &str) -> String {
    let mut ans:Vec<String> = vec![];
    let morse_codes = utils::from_morse_code();
    let encoded = encoded.split("   ");
    for code in encoded {
        let mut stack = vec![];
        for word in code.split_whitespace() { 
            let code = morse_codes.get(word).expect("Cannot fetch");
            stack.push(code.to_uppercase());
        }
        ans.push(stack.join(""))
    }
    ans.join(" ")
}

#[allow(unused)]
fn  aliter(encoded: &str) -> String {
    let morse_codes = utils::from_morse_code();
    encoded
    .split(' ')
    .map(|x| { morse_codes.get(x).unwrap_or(&" ".to_string()).clone() })
    .collect::<String>()
    .split_whitespace()
    .collect::<Vec<_>>()
    .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hey_jude() {
        assert_eq!(decode_morse(".... . -.--   .--- ..- -.. ."), "HEY JUDE");
    }
}
