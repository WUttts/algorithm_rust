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
    mut carry: i32,
) -> Option<Box<ListNode>> {
    //如果 l1 l2 为None,且无进位，则说明递归结束
    if l1.is_none() && l2.is_none() && carry == 0 {
        return None;
    }
    //递归
    Some(Box::new(ListNode {
        next: carried(
            l1.and_then(|x| {
                carry += x.val;
                x.next
            }),
            l2.and_then(|x| {
                carry += x.val;
                x.next
            }),
            carry / 10,
        ),
        val: carry % 10,
    }))
}
///循坏实现
pub fn add_two_numbers_traverse(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    if l1.is_none() && l2.is_none() {
        return None;
    }
    let (mut l1, mut l2) = (l1, l2);
    let (mut first, mut second) = (0, 0);

    let mut dummy = ListNode::new(-1);
    let mut cur = &mut dummy;
    let mut carry = 0;

    while l1.is_some() || l2.is_some() {
        if l1.as_ref().is_some() {
            first = l1.as_ref().unwrap().val;
        }
        if l2.as_ref().is_some() {
            second = l2.as_ref().unwrap().val;
        }
        l1 = l1.unwrap().as_mut().next.take();
        l2 = l2.unwrap().as_mut().next.take();
        let val = first + second + carry;
        cur.next = Some(Box::new(ListNode::new(val % 10)));
        cur = cur.next.as_mut().unwrap();
        carry = if val >= 10 { 1 } else { 0 };
    }
    dummy.next
}

///模式匹配
pub fn add_two_numbers_pattern_matching(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut res = None;
    let mut tail = &mut res;
    let mut t = (l1, l2, 0, 0); //l1,l2,sum,carry
    loop {
        t = match t {
            (None, None, _, 0) => break,
            (None, None, _, carry) => (None, None, carry, 0),
            (Some(list), None, _, carry) | (None, Some(list), _, carry)
                if list.val + carry >= 10 =>
            {
                (list.next, None, list.val + carry - 10, 1)
            }
            (Some(list), None, _, carry) | (None, Some(list), _, carry) => {
                (list.next, None, list.val + carry, 0)
            }
            (Some(l1), Some(l2), _, carry) if l1.val + l2.val + carry >= 10 => {
                (l1.next, l2.next, l1.val + l2.val + carry - 10, 1)
            }
            (Some(l1), Some(l2), _, carry) => (l1.next, l2.next, l1.val + l2.val + carry, 0),
        };
        *tail = Some(Box::new(ListNode::new(t.2)));
        tail = &mut tail.as_mut().unwrap().next;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let l1 = linkedlist![2, 4, 3];
        let l2 = linkedlist![5, 6, 4];
        let standard = linkedlist![7, 0, 8];

        let res1 = add_two_numbers_traverse(l1, l2);
        assert_eq!(standard, res1);
    }
    #[test]
    fn test_2() {
        let l1 = linkedlist![2, 4, 3];
        let l2 = linkedlist![5, 6, 4];
        let standard = linkedlist![7, 0, 8];
        assert_eq!(standard, add_two_numbers(l1, l2));
    }

    #[test]
    fn test_3() {
        let l1 = linkedlist![9, 9, 9, 9, 9, 9, 9];
        let l2 = linkedlist![9, 9, 9, 9];
        assert_eq!(linkedlist![8, 9, 9, 9, 0, 0, 0, 1], add_two_numbers(l1, l2));
    }
}
