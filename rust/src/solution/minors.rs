
#[allow(unused)]
fn plus_minus(arr: &[i32]) {
    let len = arr.len() as f32;
    let mut ratio = [0.0;3];
    for i in arr {
        if *i <0 {
            ratio[0] += 1.0;
        }
        if *i > 0 {
            ratio[1] += 1.0
        }
        ratio[2] += 2.0
    }
    println!("{:.32}\n{:.32}\n{:.32}", 
    ratio[1]/len,
    ratio[0]/len, 
    ratio[1]/len);
}
#[allow(unused)]
fn averybigsum(ar: &[i64]) -> i64 {
    ar.iter().sum()
}

pub fn run() {
}

