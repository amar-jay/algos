/*
The task is simply stated. Given an integer n (3 < n < 109), find the length of the smallest list of perfect squares which add up to n. Come up with the best algorithm you can; you'll need it!

Examples:

sum_of_squares(17) = 2
17 = 16 + 1 (16 and 1 are perfect squares).
sum_of_squares(15) = 4
15 = 9 + 4 + 1 + 1. There is no way to represent 15 as the sum of three perfect squares.
sum_of_squares(16) = 1
16 itself is a perfect square.
Time constraints:

5 easy (sample) test cases: n < 20

5 harder test cases: 1000 < n < 15000

5 maximally hard test cases: 5e8 < n < 1e9

1000 random maximally hard test cases: 1e8 < n < 1e9
*/

#[allow(unused)]
fn sum_of_squares(n: u64) -> u64 {
    //base case
    if n <= 3 {
        return n;
    }
    let p_sqs = perfect_squares(n);
    p_sqs.len() as u64
}


/**
* Finds all perfect squares less than n 
*/
fn perfect_squares(n: u64) -> Vec<u64> {
    let mut ans = vec![];
    for i in 1..(n+1) {
        if i * i <= n {
            ans.push(i)
        }
    }
    ans
}


#[allow(unused)]
fn aliter(n: u64) -> u64 {
    if (n as f64).sqrt() % 1.0 == 0.0 { 
        return 1
    }
    
    let mut temp_n = n;
    
    while temp_n % 4 == 0 {     
        temp_n = ((temp_n as f64) / 4.0).round() as u64
    }
        
    if temp_n % 8 == 7 { 
        return 4
    }
    
    let mut i = 0;
        
    while i * i < temp_n {
        let num = temp_n - i * i;
        
        if is_perfect_square(num) { 
            return 2
        }
        
        i += 1;
    }
    
    3
}

fn is_perfect_square(n: u64) -> bool { 
    (n as f64).sqrt() % 1.0 == 0.0
}

#[cfg(test)]
mod tests {
    //use super::sum_of_squares as solution;
    use super::aliter as solution;
    
    #[test]
    fn sample_tests() {
        assert_eq!(solution(15), 4, "The solution for sum_of_squares(15) is not the expected answer (4).");
        assert_eq!(solution(16), 1, "The solution for sum_of_squares(16) is not the expected answer (1).");
        assert_eq!(solution(17), 2, "The solution for sum_of_squares(17) is not the expected answer (2).");
        assert_eq!(solution(18), 2, "The solution for sum_of_squares(18) is not the expected answer (2).");
        assert_eq!(solution(19), 3, "The solution for sum_of_squares(19) is not the expected answer (3).");
    }
}
