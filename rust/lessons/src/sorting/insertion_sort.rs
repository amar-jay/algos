use std::cmp;
#[allow(unused)]
/**
## Steps
-  Traverse arr from 2nd val
-  store curr val and j as prev index
-  loop as long as j != 0 or prev val > curr val 
-  move it val by 1 index and decrease j by 1
-  curr val now idx == 0 by curr
*/
pub fn insertion_sort<T>(arr: &mut [T])
where 
    T: cmp::PartialOrd + Copy 
{
    for i in 1..arr.len()  {
        let curr = arr[i];
        let mut j = i -1;

        while j != 0 && arr[j] > curr {
            arr[j + 1] = arr[j];
            j -= 1;
        }

        // j -> 0
        arr[j] = curr;
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
        insertion_sort(&mut ve1);
        insertion_sort(&mut alp);
        assert!(is_sorted(&ve1));
        assert!(is_sorted(&alp));
    }

    #[test]
    fn ascending() {
        //pre-sorted
        let mut alp = vec!['a', 'b', 'c', 'd'];
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        insertion_sort(&mut ve2);
        insertion_sort(&mut alp);
        assert!(is_sorted(&ve2));
        assert!(is_sorted(&alp));
    }

    #[test]
    fn one_element() {
        let mut alp = ['a'];
        let mut arr = [9];
        insertion_sort(&mut arr);
        insertion_sort(&mut alp);
        assert!(is_sorted(&arr));
        assert!(is_sorted(&alp));
    }

    #[test]
    fn similar_vals() {
        let mut ve2 = vec![0, 0 , 0, 0, 1];
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
