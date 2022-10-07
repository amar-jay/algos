#[allow(unused)]
/**
* ## Steps
*   -  Traverse through arr with idx, i
*   -  Traverse through arr-i-1 with idx,j
*   -  swap elem if curr greater than prev
*/
pub fn bucket_sort<T: Ord>(arr: &mut [T]) {
   // unimplemented!();
    let n = 10;
    // create n empty buckets
    let mut bucket:Vec<Vec<T>> = vec![];
    for i in 0..n {
        bucket.push(vec![])
    }
    // Put array elements in different buckets
    todo!("Bucket sort NOT DONE");
}


#[cfg(test)]
mod tests {
    use super::super::is_sorted;
    use super::*;

    #[test]
    fn descending() {
        //descending
        let mut alp = vec!['d', 'c', 'b', 'a'];
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        bucket_sort(&mut ve1);
        bucket_sort(&mut alp);
        assert!(is_sorted(&ve1));
        assert!(is_sorted(&alp));
    }

    #[test]
    fn ascending() {
        //pre-sorted
        let mut alp = vec!['a', 'b', 'c', 'd'];
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        bucket_sort(&mut ve2);
        bucket_sort(&mut alp);
        assert!(is_sorted(&ve2));
        assert!(is_sorted(&alp));
    }

    #[test]
    fn one_element() {
        let mut alp = ['a'];
        let mut arr = [9];
        bucket_sort(&mut arr);
        bucket_sort(&mut alp);
        assert!(is_sorted(&arr));
        assert!(is_sorted(&alp));
    }

    #[test]
    fn similar_vals() {
        let mut alp = ['a', 'a', 'a', 'y'];
        let mut ve2 = vec![0, 0 , 0, 0, 1];
        bucket_sort(&mut ve2);
        bucket_sort(&mut alp);
        assert!(is_sorted(&ve2));
        assert!(is_sorted(&alp));
    }


    #[test]
    fn random() {
        let mut ve2 = vec![2, 4, 6, 8, 1, 5];
        bucket_sort(&mut ve2);
        assert!(is_sorted(&ve2));
    }

    #[test]
    fn empty() {
        let mut ve3: Vec<usize> = vec![];
        bucket_sort(&mut ve3);
        assert!(is_sorted(&ve3));
    }

}
