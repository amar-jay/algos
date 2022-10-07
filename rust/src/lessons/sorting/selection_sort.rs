#[allow(unused)]
/**
  ## Steps
  -  Tranverse through arr 
  -  find index of min val in array
  -  swap min val with curr val 
*/
fn selection_sort<T>(arr: &mut [T])
where T: PartialOrd {
    for i in 0..arr.len() {
        let mut min_idx = i;
        for i in i..arr.len() {
            if arr[min_idx] > arr[i] {
                min_idx = i;
            }
        }
        arr.swap(min_idx, i)
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
        selection_sort(&mut ve1);
        selection_sort(&mut alp);
        assert!(is_sorted(&ve1));
        assert!(is_sorted(&alp));
    }

    #[test]
    fn ascending() {
        //pre-sorted
        let mut alp = vec!['a', 'b', 'c', 'd'];
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        selection_sort(&mut ve2);
        selection_sort(&mut alp);
        assert!(is_sorted(&ve2));
        assert!(is_sorted(&alp));
    }

    #[test]
    fn one_element() {
        let mut alp = ['a'];
        let mut arr = [9];
        selection_sort(&mut arr);
        selection_sort(&mut alp);
        assert!(is_sorted(&arr));
        assert!(is_sorted(&alp));
    }

    #[test]
    fn similar_vals() {
        let mut ve2 = vec![0, 0 , 0, 0, 1];
        selection_sort(&mut ve2);
        assert!(is_sorted(&ve2));
    }


    #[test]
    fn random() {
        let mut ve2 = vec![2, 4, 6, 8, 1, 5];
        selection_sort(&mut ve2);
        assert!(is_sorted(&ve2));
    }

    #[test]
    fn empty() {
        let mut ve3: Vec<usize> = vec![];
        selection_sort(&mut ve3);
        assert!(is_sorted(&ve3));
    }

}
