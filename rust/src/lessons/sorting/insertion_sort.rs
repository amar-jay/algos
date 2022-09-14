#[allow(unused)]
pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
}

#[cfg(test)]
mod tests {
    use super::super::is_sorted;
    use super::*;

    #[test]
    fn descending() {
        //descending
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        insertion_sort(&mut ve1);
        assert!(is_sorted(&ve1));
    }

    #[test]
    fn ascending() {
        //pre-sorted
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        insertion_sort(&mut ve2);
        assert!(is_sorted(&ve2));
    }

    #[test]
    fn random() {
        let mut ve2 = vec![2, 4, 6, 8, 1, 5];
        insertion_sort(&mut ve2);
        assert!(is_sorted(&ve2));
    }

    #[test]
    fn empty() {
        let mut ve3: Vec<usize> = vec![];
        insertion_sort(&mut ve3);
        assert!(is_sorted(&ve3));
    }
}
