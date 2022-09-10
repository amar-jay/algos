
/**
* Write a function named set_alarm which receives two parameters.
* The first parameter, employed, is true whenever you are employed 
* and the second parameter, vacation is true whenever you are on vacation.
* The function should return true if you are employed and not on vacation 
* (because these are the circumstances under which you need to set an alarm).
* It should return false otherwise.
* Examples:
set_alarm(true, true) -> false
set_alarm(false, true) -> false
set_alarm(false, false) -> false
set_alarm(true, false) -> true
*/

#[allow(unused)]
fn set_alarm(employed: bool, vacation: bool) -> bool {
    match (employed, vacation) {
        (true, false) => true,
        (_, _) =>   false,
    }
}
#[cfg(test)]
mod tests {
    use super::set_alarm;

    
    #[test]
    fn test_set_alarm() {
        assert!(!set_alarm(true, true), "Fails when input is true, true");
        assert!(!set_alarm(false, true), "Fails when input is false, true");
        assert!(!set_alarm(false, false), "Fails when input is false, false");
        assert!(set_alarm(true, false), "Fails when input is true, false");
    }

}
