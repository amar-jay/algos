use  chrono::Timelike;
use chrono::NaiveDateTime;
/**
Convert Date from seconds to "%d hours %d minutes %d seconds"
*
*/
#[allow(dead_code)]
fn aliter(seconds: u64) -> String {
    let date:NaiveDateTime = NaiveDateTime::from_timestamp(seconds as i64, 0);
    let mut ans = vec![]; 

    match seconds / (60 * 60 * 24) {
        0 => {},
        1 => { ans.push("1 day".to_string())}
        dd => {
        ans.push(format!("{} days", dd));
        }
    }


    match date.hour(){
        0 => {},
        1 => { ans.push("1 hour".to_string())}
        _ => {
        ans.push(format!("{} hours", date.hour()));
        }
    }

    match date.minute(){
        0 => {},
        1 => { ans.push(format!("1 minute"))},
        _ => { ans.push(format!("{} minutes", date.minute())); }
    }


    match date.second() {
        0 => {},
        1 => ans.push(format!("1 second")),
        x => ans.push(format!("{} seconds", x)),
        
    }

    if ans.len() > 2 {
        let x = ans[0..(ans.len() - 1)].join(", ");
            return format!("{} and {}", x, ans.last().unwrap());
    }
    if ans.len() == 0 {
        return "now".to_string()
    }
        return ans.join(" and ")
}

#[allow(dead_code)]
fn main_solution(seconds: u64) -> String {
    // Complete this function
    let date:NaiveDateTime = NaiveDateTime::from_timestamp(seconds as i64, 0);
    let mut ans = String::from(""); 

    if date.hour() != 0 {
        if date.hour() == 1 { ans.push_str(&format!("1 hour and "))}
        else {
        ans.push_str(&format!("{} hours and ", date.hour()));
        }
    }

    if date.minute() != 0 {
        if date.minute() == 1 { ans.push_str(&format!("1 minute and "))}
        else {
        ans.push_str(&format!("{} minutes and ", date.minute()));
        }
    }


    if date.second() != 0 {
        if date.second() == 1 { ans.push_str(&format!("1 second"))}
        else {
        ans.push_str(&format!("{} seconds", date.second()));
        }
    }


    return ans.trim().to_string()
}

fn format_duration(seconds: u64) -> String {
    aliter(seconds)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advanced() {

        assert_eq!(format_duration(58272190), "674 days, 10 hours, 43 minutes and 10 seconds");
        assert_eq!(format_duration(58272180), "674 days, 10 hours and 43 minutes");
        assert_eq!(format_duration(58272181), "674 days, 10 hours, 43 minutes and 1 second");
    }
    #[test]
    fn test_basic() {
        assert_eq!(format_duration(1), "1 second");
        assert_eq!(format_duration(62), "1 minute and 2 seconds");
        assert_eq!(format_duration(120), "2 minutes");
        assert_eq!(format_duration(3600), "1 hour");
        assert_eq!(format_duration(3662), "1 hour, 1 minute and 2 seconds");
    }
}
