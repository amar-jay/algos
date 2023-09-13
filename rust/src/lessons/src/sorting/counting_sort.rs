#[allow(unused)]
/**
## NOTE: Counting sort if only used when you know the range of values beforehand

## Steps

- 
*/
pub fn counting_sort(arr: &mut [i32]) {
    // TODO: Change it to work for characters as well
    // For this the range of possible values is 0-5
    let mut counter = vec![0;10];
    for i in 0..arr.len() {
       counter[arr[i] as usize]+= 1; 
    }

    let mut ans = vec![];
    for (idx, &i) in counter.iter().enumerate() {
        for x  in 0..i {
            ans.push(idx+1)
        }
    }

    let mut id:usize = 0;
    for i in ans {

        if let Some(num) = arr.get_mut(id) {
        *num = i as i32;
        }
        id += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::super::is_sorted;
    use super::*;

    #[test]
    fn descending() {
        //descending
        let mut ve1 = vec![5, 4, 3, 2, 1];
        counting_sort(&mut ve1);
        assert!(is_sorted(&ve1));
    }

    #[test]
    fn ascending() {
        //pre-sorted
        let mut ve2 = vec![1, 2, 3, 4, 5];
        counting_sort(&mut ve2);
        assert!(is_sorted(&ve2));
    }

    #[test]
    fn one_element() {
        let mut arr = [4];
        counting_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn similar_vals() {
        let mut ve2 = vec![0, 2 , 0, 0, 1];
        counting_sort(&mut ve2);
        assert!(is_sorted(&ve2));

    }


    #[test]
    fn random() {
        let mut ve2 = vec![2, 4, 0, 1, 5];
        counting_sort(&mut ve2);
        assert!(is_sorted(&ve2));
    }

    #[test]
    fn empty() {
        let mut ve3: Vec<i32> = vec![];
        counting_sort(&mut ve3);
        assert!(is_sorted(&ve3));
    }

}
