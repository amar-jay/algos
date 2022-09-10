pub fn time_conversion(s: &str) -> String {
    let (hour, min, sec, am_or_pm) = (&s[0..2], &s[3..5], &s[6..8], &s[9..10]);
    let hh = hour.parse::<isize>().expect("parsing error")  + match (hour, am_or_pm) {
        ("12", "AM") => -12,
        (_, "AM") => 0,
        ("12", "PM") => 0,
        _ => 12
    };
   
   format!("{:02}:{:02}:{02}", hh, min, sec)
    
}

pub fn run() {
    println!("Data Structure: String Formatting");
    println!("{}", time_conversion("10:12:30PM"));
}
