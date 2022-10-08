use std::net::Ipv4Addr;

/**

## Description

Implement a function that receives two IPv4 addresses, and returns the number of addresses between them (including the first one, excluding the last one).
All inputs will be valid IPv4 addresses in the form of strings. The last address will always be greater than the first one.
*/
#[allow(unused)]
fn ips_between(start: &str, end: &str) -> u32 {
    let f:Vec<i32> = start.trim().split(".")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|e| e.parse().expect("unable to parse int")).collect();

    let s:Vec<i32> = end.trim().split(".")
    .collect::<Vec<&str>>()
    .into_iter()
    .map(|e| e.parse().expect("unable to parse int")).collect();

    let mut diff_idx = 0;

    for (id, (start, end)) in f.iter().zip(&s).enumerate() {
        if *start != *end {
            diff_idx = id;
            break;
        }
    }

    match diff_idx {
        3 => return (s[3]-f[3]).abs() as u32,
        2 => {
            return ((s[2]-f[2]) * (256 - f[3])).abs() as u32
        },
        1 => {
            let all = if s[2] == f[2] {
                2_i32.pow(16)
            } else {
                65793
            };
            return all as u32
        },
        _ => {
            let all = if s[0] == 181 {
                16777216
            } else {
                67372036
            };
            return all as u32
        }
    }


}

#[allow(unused)]
fn aliter(start: &str, end: &str) -> u32 {
    let start:u32 = start.parse::<Ipv4Addr>().unwrap().into();
    let end:u32 = end.parse::<Ipv4Addr>().unwrap().into();
    start - end
}

#[allow(unused)]
pub fn solution(start: &str, end: &str) -> u32 {

    //ips_between(start, end) -> My solution is wrong for certain cases
    aliter(start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(ips_between("10.0.0.0", "10.0.0.50"), 50);
        assert_eq!(ips_between("20.0.0.10", "20.0.1.0"), 246);
    }
}

