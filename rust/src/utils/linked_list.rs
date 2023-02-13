///To Create ListNode. val -> next -> next
#[derive(Debug, Eq, PartialEq)]
pub struct ListNode
{
    pub curr: i32,
    pub next: Option<Box<ListNode>>

}

impl Copy for ListNode {
    fn copy(&self) -> Self {
        Self {
            curr: self.curr,
            next: self.next,
        }
    }
}
//impl <T>std::cmp::PartialEq for ListNode<T> {}
impl ListNode {
    #[allow(unused)]
    pub fn new(val:i32) -> ListNode {
        ListNode { curr: val, next: None}
    }
    // list node to vec
    pub fn to_vec(&self) -> Vec<i32> {
        let mut v = Vec::new();
        v.push(self.curr);
        
        let mut curr = self.next.as_ref();
        while let Some(node) = curr {
            v.push(node.curr);
            curr = node.next.as_ref();
        }
        v
    }
    
}

/// Convert Vector to ListNode
#[allow(unused)]
pub fn to_list(vec: &Vec<i32>) -> Option<Box<ListNode>> {

    let mut prev = None;
    for i in vec.iter().rev() {
        let mut list = ListNode::new(*i);
        list.next = prev;
        prev = Some(Box::new(list))
    }

    prev
}


