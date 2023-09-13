#[allow(unused)]
/**
* # Quick Sort
* Quick sort is a comparison sorting algorithm that uses a divide and conquer strategy.
* It is a sorting algorithm that uses divide and conquer approach.
* ## Steps
*   - 
*   -
*   -
*/
fn quick_sort<T: Ord>(arr: &mut Vec<T>) -> Vec<T>
where T: Ord + Copy + Clone {
    let len = arr.len();

    if len <= 1 {
        return arr.to_vec();
    }

    let pivot = *arr.first().unwrap();
    let mut greater_than_pivot = arr.to_vec().into_iter().filter(|&x| x > pivot).collect::<Vec<T>>();
    let mut less_than_pivot = arr.to_vec().into_iter().filter(|&x| x < pivot).collect::<Vec<T>>();
    let mut sorted = vec![];
    sorted.append(&mut quick_sort(&mut less_than_pivot));
    sorted.push(pivot);
    sorted.append(&mut quick_sort(&mut greater_than_pivot));
    return sorted;
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
        let ve1 = quick_sort(&mut ve1);
        let alp = quick_sort(&mut alp);
        assert_eq!(vec![1, 2, 3, 4, 5, 6], ve1);
        assert_eq!(vec!['a','b', 'c', 'd'], alp);
        assert!(is_sorted(&alp));
    }

    #[test]
    fn ascending() {
        //pre-sorted
        let mut alp = vec!['a', 'b', 'c', 'd'];
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        let ve2 = quick_sort(&mut ve2);
        let alp = quick_sort(&mut alp);
        assert!(is_sorted(&ve2));
        assert!(is_sorted(&alp));
    }

    #[test]
    fn one_element() {
        let mut alp = vec!['a'];
        let mut arr = vec![9];
        let arr = quick_sort(&mut arr);
        let alp = quick_sort(&mut alp);
        assert!(is_sorted(&arr));
        assert!(is_sorted(&alp));
    }

    #[test]
    fn similar_vals() {
        let mut alp = vec!['a', 'a', 'a', 'y'];
        let mut ve2 = vec![0, 0 , 0, 0, 1];
        quick_sort(&mut ve2);
        quick_sort(&mut alp);
        assert!(is_sorted(&ve2));
        assert!(is_sorted(&alp));
    }


    #[test]
    fn random() {
        let mut ve2 = vec![2, 4, 6, 8, 1, 5];
        let ve2 = quick_sort(&mut ve2);
        assert_eq!(vec![1, 2, 4, 5, 6, 8], ve2);
    }

    #[test]
    fn empty() {
        let mut ve3: Vec<usize> = vec![];
        quick_sort(&mut ve3);
        assert!(is_sorted(&ve3));
    }

}
