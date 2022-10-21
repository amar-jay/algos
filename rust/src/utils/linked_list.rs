///To Create ListNode. val -> next -> next
#[derive(Debug, Eq, PartialEq)]
pub struct ListNode
{
    pub curr: i32,
    pub next: Option<Box<ListNode>>

}

//impl <T>std::cmp::PartialEq for ListNode<T> {}
impl ListNode {
    #[allow(unused)]
    pub fn new(val:i32) -> ListNode {
        ListNode { curr: val, next: None}
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


