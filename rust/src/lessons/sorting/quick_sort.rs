#[allow(unused)]
/**
* ## Steps
*   - 
*   -
*   -
*/
pub fn quick_sort<T: Ord>(arr: &mut [T]) {
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
        quick_sort(&mut ve1);
        quick_sort(&mut alp);
        assert!(is_sorted(&ve1));
        assert!(is_sorted(&alp));
    }

    #[test]
    fn ascending() {
        //pre-sorted
        let mut alp = vec!['a', 'b', 'c', 'd'];
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        quick_sort(&mut ve2);
        quick_sort(&mut alp);
        assert!(is_sorted(&ve2));
        assert!(is_sorted(&alp));
    }

    #[test]
    fn one_element() {
        let mut alp = ['a'];
        let mut arr = [9];
        quick_sort(&mut arr);
        quick_sort(&mut alp);
        assert!(is_sorted(&arr));
        assert!(is_sorted(&alp));
    }

    #[test]
    fn similar_vals() {
        let mut alp = ['a', 'a', 'a', 'y'];
        let mut ve2 = vec![0, 0 , 0, 0, 1];
        quick_sort(&mut ve2);
        quick_sort(&mut alp);
        assert!(is_sorted(&ve2));
        assert!(is_sorted(&alp));
    }


    #[test]
    fn random() {
        let mut ve2 = vec![2, 4, 6, 8, 1, 5];
        quick_sort(&mut ve2);
        assert!(is_sorted(&ve2));
    }

    #[test]
    fn empty() {
        let mut ve3: Vec<usize> = vec![];
        quick_sort(&mut ve3);
        assert!(is_sorted(&ve3));
    }

}