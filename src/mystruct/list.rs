use std::cmp::Ordering;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn create_linkedlist(nums: Vec<Option<i32>>) -> Option<Box<ListNode>> {
    let mut res = ListNode::new(-1);
    let mut cur = &mut res;
    for i in nums {
        if let Some(num) = i {
            cur.next = Some(Box::new(ListNode::new(num)));
            cur = cur.next.as_mut().unwrap();
        }
    }
    res.next
}
///
/// 快速创建链表
#[macro_export]
macro_rules! linkedlist {
    () => {
        None
    };
    ($($e:expr),*) => {
        {
            let vec = vec![$(stringify!($e)), *];
            let vec = vec.into_iter().map(|v| v.parse::<i32>().ok()).collect::<Vec<_>>();
            crate::mystruct::list::create_linkedlist(vec)
        }
    };
    ($($e:expr,)*) => {(linkedlist![$($e),*])};
}
