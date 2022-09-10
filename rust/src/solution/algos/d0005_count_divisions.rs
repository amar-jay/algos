
/**
* Count the number of divisors of a positive integer n.
* Random tests go up to n = 500000.
*
* Examples (input --> output)
*
*
*/

#[allow(unused)]
fn divisors(n: i32) -> i32 {
    let rt = (n as f32).sqrt() as i32;
    let mut vals = vec![];
    for i in 1..(n){

        if n%i == 0 {
            vals.push(i);
        }
        if i != n / i {
            vals.push(n / i);
        }
    }

    vals.len() as i32
}

#[cfg(test)]
mod tests {
    use super::divisors;

    #[test]
    fn sample_tests() {
        assert_eq!(divisors(1), 1);
        assert_eq!(divisors(4), 3);
        assert_eq!(divisors(5), 2);
        assert_eq!(divisors(12), 6);
        assert_eq!(divisors(25), 3);
        assert_eq!(divisors(4096), 13);
    }
}
