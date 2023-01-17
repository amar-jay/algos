//! Binary Search 
//! ============
//! 
//! @complexity: O(n) (on average )
//! @complexity: O(n) (worst case)

use std::cmp::Ordering;
#[allow(unused)]
pub fn quick_select<T>(arr: &[T], n: T) -> usize
where T:Ord + Copy  {
    let (mut low, mut high) = (0, arr.len());

    while (low <= high) {
        let mid = low + (high - low) / 2;
        match (n.cmp(&arr[mid])) {
            Ordering::Less => high = mid - 1,
            Ordering::Greater => low = mid + 1,
            Ordering::Equal =>  return mid
        }
    }
    0
}
#[cfg(test)]
mod tests {
    use super::quick_select;

    #[test]
    fn test_easy() {
        let arr = [3,4, 5, 6, 8, 9];
        let ans = quick_select(&arr, 5);
        assert_eq!(ans, 2);
    }
}
