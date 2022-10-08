/**
## Description

You are given a list/array which contains only integers (positive and negative). Your job is to sum only the numbers that are the same and consecutive. The result should be one list.
Extra credit if you solve it in one line. You can assume there is never an empty list/array and there will always be an integer.

Same meaning: 1 == 1
1 != -1
*/

/// Sums the numbers that are the same and consecutive.
#[allow(unused)]
fn sum_consecutives(numbers: &Vec<i32>) -> Vec<i32> {
    let mut ans:Vec<i32> = vec![];
    let mut prev= 112393303; // might break but highly improbable

    for i in numbers {
        if *i != prev {
            prev = *i;
            ans.push(*i);
        } else {
            let last = ans.pop().expect("popping failed");
            ans.push(last+prev)
        }
    }
    ans

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let input = vec![0];
        let expected = vec![0];

        assert_eq!(sum_consecutives(&input), expected);
}
    #[test]
    fn test_sample() {
        let input = vec![1, 4, 4, 4, 0, 4, 3, 3, 1];
        let expected = vec![1, 12, 0, 4, 6, 1];
        assert_eq!(sum_consecutives(&input), expected);

    }
    #[test]
    fn test_negative() {
        let input = vec![-5, -5, 7, 7, 12, 0];
        let expected = vec![-10, 14, 12, 0];
        assert_eq!(sum_consecutives(&input), expected);
    }
}
