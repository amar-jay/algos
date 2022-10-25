/** https://www.geeksforgeeks.org/write-a-program-to-Reverse-an-array-or-string/
 * This function will accept an array and
 * Reverse its elements and returns the inverted array
 * @param {Array} arr array with elements of any data type
 * @returns {Array} array with inverted elements
 */


#[allow(unused)]
pub fn reverse_arr<T>(arr:&mut [T]) -> &[T] 
where T:Copy + Ord{
    let len = arr.len();
    for i in 0..(len/2) {
        arr.swap(i, len - i - 1);
    }
    arr
}

#[cfg(test)]
mod test {
    #[test]
    fn test_empty_array() {
        let mut arr: [i32; 0] = [];
        let result = super::reverse_arr(&mut arr);
        assert_eq!(result, &[]);
    }

    #[test]
    fn test_reverse_arr() {
        let mut arr = [1, 2, 3, 4, 5];
        let result = super::reverse_arr(&mut arr);
        assert_eq!(result, &[5, 4, 3, 2, 1]);
    }

}
