#![allow(unused, dead_code)]

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct ListNode
{
    pub val: i32,
    pub next: Option<Box<ListNode>>

}

impl ListNode {
    #[inline]
    pub fn new(val:i32) -> ListNode {
        ListNode { val, next: None}
    }

    // list node to vec
    pub fn to_vec(&self) -> Vec<i32> {
        let mut v = Vec::new();
        v.push(self.val);
        
        let mut val = self.next.as_ref();
        while let Some(node) = val {
            v.push(node.val);
            val = node.next.as_ref();
        }
        v
    }
    
}

/// Convert Vector to ListNode
pub fn to_list(vec: &Vec<i32>) -> Option<Box<ListNode>> {

    let mut prev = None;
    for i in vec.iter().rev() {
        let mut list = ListNode::new(*i);
        list.next = prev;
        prev = Some(Box::new(list))
    }

    prev
}
/// Convert ListNode to Vector
pub fn to_vec(vec: &ListNode) -> Vec<i32> {
    let mut v = Vec::new();
    v.push(vec.val);
    
    let mut curr = vec.next.as_ref();
    while let Some(node) = curr {
        v.push(node.val);
        curr = node.next.as_ref();
    }
    v
}
