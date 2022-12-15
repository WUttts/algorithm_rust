use crate::mystruct::list::ListNode;

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    //传入两条链表，和进位值
    carried(l1, l2, 0)
}

fn carried(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    carry: i32,
) -> Option<Box<ListNode>> {
    //如果 l1 l2 为None,且无进位，则说明递归结束
    if l1.is_none() && l2.is_none() && carry == 0 {
        return None;
    }
    //处理递归
    
    
    None
}
