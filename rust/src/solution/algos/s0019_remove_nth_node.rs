/*!
Given the head of a linked list, remove the nth node 
from the end of the list and return its head.
## Example 1:
```
Input: head = [1,2,3,4,5], n = 2
Output: [1,2,3,5]
```
## Example 2:
```
Input: head = [1], n = 1
Output: []
```
## Example 3:
```
Input: head = [1,2], n = 1
Output: [1]
```
*/
use crate::utils::linked_list::ListNode;
#[allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut len = 0;
        let mut curr = &head;
        while let Some(node) = curr {
            len += 1;
            curr = &node.next;
        }

        let mut curr = &mut head;
        let mut i = 0;

        if len == n {
            return head.and_then(|x| x.next);
        }

        while let Some(node) = curr {
            i += 1;
            if i == len - n {
                let next = node.next.take();
                node.next = next.and_then(|x| x.next);
                break;
            }
            curr = &mut node.next;
        }

        head

    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::utils::linked_list::to_list;
    
//    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    #[test]
    fn test_simple() {
        let inp = vec![1, 2, 3, 4, 5];
        let target = 2;
        let expected = vec![1, 2, 3, 5];
        let out = Solution::remove_nth_from_end(to_list(&inp), target);
        // vec comparison
        assert_eq!(out.as_ref().unwrap().to_vec(), expected);
    }
    
    #[test]
    fn test_second() {
        let inp = vec![2,2,2,2,2];
        let target = 4;
        let expected:Vec<i32> = vec![2,2,2,2];
        let out = Solution::remove_nth_from_end(to_list(&inp), target);
        assert_eq!(out.as_ref().unwrap().to_vec(), expected);
    }

    #[test]
    fn test_single() {
        let inp = vec![1];
        let target = 1;
        let expected:Vec<i32> = vec![];
        let out = Solution::remove_nth_from_end(to_list(&inp), target);
        assert_eq!(out, to_list(&expected));
    }
}
