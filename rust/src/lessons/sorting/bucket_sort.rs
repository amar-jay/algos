#[allow(unused)]
/**
 * ## BucketSort implementation.
 *
 * Wikipedia says: Bucket sort, or bin sort, is a sorting algorithm that works by distributing the elements of an array
 * into a number of buckets. Each bucket is then sorted individually, either using a different sorting algorithm, or by
 * recursively applying the bucket sorting algorithm. It is a distribution sort, and is a cousin of radix sort in the
 * most to least significant digit flavour. Bucket sort is a generalization of pigeonhole sort. Bucket sort can be
 * implemented with comparisons and therefore can also be considered a comparison sort algorithm. The computational
 * complexity estimates involve the number of buckets.
 *
 * @see https://en.wikipedia.org/wiki/Bucket_sort#:~:text=Bucket%20sort%2C%20or%20bin%20sort,applying%20the%20bucket%20sorting%20algorithm.&text=Sort%20each%20non%2Dempty%20bucket.
 *
 * Time Complexity of Solution:
 * Best Case O(n); Average Case O(n); Worst Case O(n)
 *
 * @param {number[]} list The array of numbers to be sorted.
 * @param {number} size The size of the buckets used. If not provided, size will be 5.
 * @return {number[]} An array of numbers sorted in increasing order.
*/
pub fn bucket_sort(arr: &mut [i32], size: usize) {
    // create n empty buckets
    if size <= 0 {
        return;
    }
    let size = size as i32;
    // find min and max values
    let min = arr.iter().min().unwrap();
    let max = arr.iter().max().unwrap();

    // how many buckets we need
    let count = (max - min) / size + 1;
    let mut bucket:Vec<Vec<i32>> = vec![];

    // create buckets
    for i in 0..count {
        bucket.push(vec![])
    }

    // Put array elements in different buckets
    for i in 0..arr.len() {
        let index = (arr[i] - min) / size;
        bucket[index as usize].push(arr[i]);
    }

    let mut a = bucket.into_iter().flat_map(|mut b| {
        b.sort();
        b.into_iter()
    }).collect::<Vec<_>>();

    for i in 0..arr.len() {
        arr[i] = a[i];
    }
}


#[cfg(test)]
mod tests {
    use super::super::is_sorted;
    use super::*;

    #[test]
    fn descending() {
        //descending
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        bucket_sort(&mut ve1, 6);
        assert!(is_sorted(&ve1));
    }

    #[test]
    fn ascending() {
        //pre-sorted
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        bucket_sort(&mut ve2, 6);
        assert!(is_sorted(&ve2));
    }

    #[test]
    fn one_element() {
        let mut arr = [9];
        bucket_sort(&mut arr, 1);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn similar_vals() {
        let mut ve2 = vec![0, 0 , 0, 0, 1];
        bucket_sort(&mut ve2, 5);
        assert!(is_sorted(&ve2));
    }


    #[test]
    fn random() {
        let mut ve2 = vec![2, 4, 6, 8, 1, 5];
        bucket_sort(&mut ve2, 6);
        assert!(is_sorted(&ve2));
    }

    #[test]
    fn empty() {
        let mut ve3: Vec<i32> = vec![];
        bucket_sort(&mut ve3, 0);
        assert!(is_sorted(&ve3));
    }

}
