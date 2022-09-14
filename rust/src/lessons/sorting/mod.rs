use std::cmp;

mod bubble_sort;
mod insertion_sort;


#[allow(unused)]
pub fn is_sorted<T>(arr: &[T]) -> bool
where
    T: cmp::PartialOrd,
{
    if arr.is_empty() {
        return true;
    }

    let mut prev = &arr[0];

    for item in arr.iter().skip(1) {
        if prev > item {
            return false;
        }

        prev = item;
    }

    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_sorted() {
        use super::is_sorted;

        assert!(is_sorted(&[] as &[usize]));
        assert!(is_sorted(&["a"]));
        // pre-sorted values
        // different values
        assert!(is_sorted(&[1, 2, 3]));
        assert!(is_sorted(&[0, 3, 4]));
        // equal values
        assert!(is_sorted(&[1, 1, 1]));
        assert!(!is_sorted(&[5, 1, 1, 5]));
        //unsorted
        assert!(!is_sorted(&[1, 1, 0, 5]));
        // negatives
        assert!(!is_sorted(&[2, 3, 0, -5]));
    }
}
