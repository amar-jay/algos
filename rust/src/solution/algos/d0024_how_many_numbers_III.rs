/**
We want to generate all the numbers of three digits where:

the sum of their digits is equal to 10
their digits are in increasing order (the numbers may have two or more equal contiguous digits)
The numbers that fulfill these constraints are: [118, 127, 136, 145, 226, 235, 244, 334]. 
There're 8 numbers in total with 118 being the lowest and 334 being the greatest.
*/
fn dig(d: u8, start:u64) -> Vec<Vec<u64>> 
{

    if d == 1 {
        let mut ans = vec![];
        for x in start..10 {
            ans.push(vec![x]);
        }
        return ans;
    } else{
        let mut ans: Vec<Vec<u64>>  = vec![];
        for x in start..10{
            for mut y in dig(d - 1, x){
                let mut f = vec![x];
                f.append(&mut y);
                ans.push(f);
            }
        }
        return ans; 
    }
}
fn find_all(sum_dig: u8, digs: u8) -> Option<(usize, u64, u64)> {
    let mut xs = vec![];
  for x in dig(digs, 1) {
        if x.iter().sum::<u64>() == sum_dig.into() {
            xs.push(x)
        }
    } 
    if xs.len() < 1 {
        return None;
    } else {
        xs.sort();
        match (xs.first(), xs.last()) {
            (Some(min), Some(max)) => {
                let stringify = |x:&Vec<u64>| x.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join("").parse().unwrap();
                let f:u64 = stringify(min);
                let s:u64 = stringify(max);

                return Some((xs.len(), f,s ))
            },
            (_, _) => return None,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::find_all;

    #[test]
    fn sample_tests() {
        assert_eq!(find_all(10, 3), Some((8, 118, 334)));
        assert_eq!(find_all(27, 3), Some((1, 999, 999)));
        assert_eq!(find_all(84, 4), None);
        assert_eq!(find_all(35, 6), Some((123, 116999, 566666)));
    }
}
