/*!

 Find the closest prime number under a certain integer n 
 that has the maximum possible amount of even digits.

For n = 1000, the highest prime under 1000 is 887, having two even digits (8 twice)

Naming f(), the function that gives that prime, the above case and others will be like the following below.

f(1000) ---> 887 (even digits: 8, 8)

f(1210) ---> 1201 (even digits: 2, 0)

f(10000) ---> 8887

f(500) ---> 487

f(487) ---> 467
Features of the random tests:

```
Number of tests = 28
1000 <= n <= 5000000
```
Enjoy it!!
 
! */

fn is_prime(n: u64) -> bool {
    if n == 2 {
        return true;
    }

    if n % 2 == 0 || n < 2 {
        return false;
    }

    (3..n).step_by(2).all(|i| n % i != 0)
}


fn num_of_even_digits(n: u64) -> usize {
    let mut count = 0;
    n.to_string().chars().for_each(|c| {
        if c == '0' || c == '2' || c == '4' || c == '6' || c == '8' {
            count += 1;
        }
    });
    count
}

#[allow(dead_code)]
fn main(n: u64) -> u64 {
    let (mut max, mut max_count) = (0, 0);
    (2..n).rev().for_each(|i| {
        if is_prime(i) {
            let count = num_of_even_digits(i);
            if count > max_count {
                max = i;
                max_count = count;
            }
        }
    });
    max
}


#[cfg(test)]
mod tests {
    use super::main as f;

    /// A test to find the margest prime number under 1000 that has the maximum possible amount of even digits.
    #[test]
    fn find_largest_prime() {
        assert_eq!(f(1000), 887);
        assert_eq!(f(1210), 1201);
        assert_eq!(f(10000), 8887);
        assert_eq!(f(500), 487);
        assert_eq!(f(487), 467);
    }

}
