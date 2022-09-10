
/**
*You will be given a list of strings.
* You must sort it alphabetically (case-sensitive, 
* and based on the ASCII values of the chars) and then return the first value.
*
* The returned value must be a string, and have "***" between each of its letters.

You should not remove or add elements from/to the array.
*
*/


#[allow(unused)]
fn two_sort(arr: &[&str]) -> String {
    let mut arr = arr.to_vec();

    arr.sort_by(|&a,&b| a.chars().cmp(&mut b.chars()));
    let first = arr[0].split("").collect::<Vec<&str>>();

    first[1..(first.len()-1)].join("***")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test_cases() {
        assert_eq!(two_sort(&["bitcoin", "take", "over", "the", "world", "maybe", "who", "knows", "perhaps"]), "b***i***t***c***o***i***n");
        assert_eq!(two_sort(&["turns", "out", "random", "test", "cases", "are", "easier", "than", "writing", "out", "basic", "ones"]), "a***r***e");
    }
}
