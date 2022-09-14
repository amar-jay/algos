#[allow(unused)]
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    //Traverse through arr with idx, i
    // Traverse through arr-i-1 with idx,j
    // swap elem if curr greater than prev
    for i in 0..arr.len() {
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            } 
        }
    }
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
        bubble_sort(&mut ve1);
        bubble_sort(&mut alp);
        assert!(is_sorted(&ve1));
        assert!(is_sorted(&alp));
    }

    #[test]
    fn ascending() {
        //pre-sorted
        let mut alp = vec!['a', 'b', 'c', 'd'];
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        bubble_sort(&mut ve2);
        bubble_sort(&mut alp);
        assert!(is_sorted(&ve2));
        assert!(is_sorted(&alp));
    }

    #[test]
    fn one_element() {
        let mut alp = ['a'];
        let mut arr = [9];
        bubble_sort(&mut arr);
        bubble_sort(&mut alp);
        assert!(is_sorted(&arr));
        assert!(is_sorted(&alp));
    }

    #[test]
    fn similar_vals() {
        let mut alp = ['a', 'a', 'a', 'y'];
        let mut ve2 = vec![0, 0 , 0, 0, 1];
        bubble_sort(&mut ve2);
        bubble_sort(&mut alp);
        assert!(is_sorted(&ve2));
        assert!(is_sorted(&alp));
    }


    #[test]
    fn random() {
        let mut ve2 = vec![2, 4, 6, 8, 1, 5];
        bubble_sort(&mut ve2);
        assert!(is_sorted(&ve2));
    }

    #[test]
    fn empty() {
        let mut ve3: Vec<usize> = vec![];
        bubble_sort(&mut ve3);
        assert!(is_sorted(&ve3));
    }

}
