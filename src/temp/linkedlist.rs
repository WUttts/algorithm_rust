use std::collections::BinaryHeap;

use crate::mystruct::list::ListNode;

//合并两条升序链表
pub fn merge_two_lists_1(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let (mut list1, mut list2) = (list1, list2);
    let mut dummy = ListNode::new(-1);
    let mut cur = &mut dummy;

    while let (Some(l1), Some(l2)) = (&list1, &list2) {
        let l = if l1.val < l2.val {
            &mut list1
        } else {
            &mut list2
        };
        cur.next = l.take();
        cur = cur.next.as_mut().unwrap();
        *l = cur.next.take();
    }
    cur.next = list1.or(list2);

    dummy.next
}

pub fn merge_two_lists_2(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, None) => None,
        (Some(l), None) => Some(l),
        (None, Some(r)) => Some(r),
        (Some(mut l), Some(mut r)) => {
            if l < r {
                l.next = merge_two_lists_2(l.next, Some(r));
                Some(l)
            } else {
                r.next = merge_two_lists_2(Some(l), r.next);
                Some(r)
            }
        }
    }
}

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    if lists.is_empty() {
        return None;
    }
    let mut dummy = ListNode::new(-1);
    let mut cur = &mut dummy;
    let mut heap = BinaryHeap::new();

    for list in lists {
        if let Some(node) = list {
            heap.push(node);
        }
    }

    while let Some(mut node) = heap.pop() {
        if let Some(v) = node.next.take() {
            heap.push(v);
        }
        cur.next = Some(node);
        cur = cur.next.as_mut().unwrap();
    }

    dummy.next
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut new_head = None;
    while let Some(mut node) = head {
        head = node.next.take();
        node.next = new_head;
        new_head = Some(node);
    }
    new_head
}

pub fn reverse_list2(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    if left == 1 {
        //反转前N个
        let mut right = right;

        let mut new_head = None;
        let mut tail = None;
        while let Some(mut node) = head {
            if right == 0 {
                tail = Some(node);
                break;
            }
            head = node.next.take();
            node.next = new_head;
            new_head = Some(node);
            right -= 1;
        }
        let mut cur = &mut new_head;
        while cur.is_some() {
            cur = &mut cur.as_mut().unwrap().next;
        }
        cur.as_mut().unwrap().next = tail;
        return new_head;
    }
    head.as_mut().unwrap().next =
        reverse_list2(head.as_mut().unwrap().next.take(), left - 1, right - 1);
    head
}

pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut dummy = ListNode::new(-1);
    let mut tail = &mut dummy;
    while head.is_some() {
        let (reverse_head, new_head) = reverse_n(head, k);
        head = new_head;
        tail.next = reverse_head;
        while tail.next.as_ref().is_some() {
            tail = tail.next.as_mut().unwrap();
        }
    }

    dummy.next
}

fn reverse_n(
    head: Option<Box<ListNode>>,
    k: i32,
) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
    //反转前N个
    let mut right = k;
    let mut head = head;
    let mut pre = head.as_ref();
    for _ in 0..k {
        if let Some(node) = pre {
            pre = node.next.as_ref();
        } else {
            return (head, None);
        }
    }

    let mut new_head = None;
    let mut tail = None;
    while let Some(mut node) = head {
        if right == 0 {
            tail = Some(node);
            break;
        }
        head = node.next.take();
        node.next = new_head;
        new_head = Some(node);
        right -= 1;
    }

    return (new_head, tail);
}

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return head;
    }
    let mut head = head;
    let mut node = head.as_mut().unwrap();
    let mut duplicate = node.val;

    while let Some(next) = node.next.take() {
        // 先夺
        if next.val != duplicate {
            duplicate = next.val; // 更新下次比较值
            node.next = Some(next); // 放回去
            node = node.next.as_mut().unwrap(); // 慢指针前进
            continue;
        }
        node.next = next.next;
    }
    head
}


pub fn recursive_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn reverse(
        head: Option<Box<ListNode>>,
        prev: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if let Some(mut node) = head {
            let tail = node.next.take();
            node.next = prev;

            return reverse(tail, Some(node));
        }
        prev
    }

    reverse(head, None)
}


#[cfg(test)]
mod tests {
    use crate::linkedlist;

    use super::*;

    #[test]
    fn test_delete_duplicates() {
        let head = linkedlist![1, 1, 2, 2, 3, 4];
        let res = delete_duplicates(head);
        println!("{:?}", res);
    }
}
