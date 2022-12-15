use crate::mystruct::list::ListNode;
use std::collections::BinaryHeap;

/// # 双指针链表题系列
/// ### [合并两条有序链表](https://leetcode.cn/problems/merge-two-sorted-lists/)
/// **关键字:**`虚拟头结点`、`新链表`
/// #### 经典解法:
pub fn merge_two_lists_1(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut list1 = list1;
    let mut list2 = list2;
    let mut dummy = ListNode::new(-1);
    let mut cur = &mut dummy;

    while let (Some(l1), Some(l2)) = (&list1, &list2) {
        let l = if l1.val < l2.val {
            &mut list1
        } else {
            &mut list2
        };
        //试想一下，l.take会把值拿出来，并且本身变为None.
        //为什么不直接 cur.next = l;l 是一个可变引用，而只有take()能把值拿出来。
        cur.next = l.take();
        cur = cur.next.as_mut().unwrap();
        //由于此时l的值可能是 list1 或者 list2 ,但不管是哪个，它们的next的变为了None.
        //所以需要重新接上，此时解引用后，l -> list1 or list2
        //那为什么布不直接 *l = cur.next?,而需要take()出来
        //因为此时cur.next 指向的值在堆上，如果我们赋值给另一个 值，意味着我们需要把它的所有权移出，或者复制一份给新变量
        //但是，Rust在没实现Copy时，是无法移动所有权的，这时候就会出现，一个值既属于cur.next，又属于 *l。
        // 所以就报错了，因此需要take()出来，并在原地留一个值。
        *l = cur.next.take();
    }
    cur.next = list1.or(list2);
    dummy.next
}

///#### 模式匹配
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, None) => None,
        (Some(l), None) => Some(l),
        (None, Some(r)) => Some(r),
        (Some(mut l), Some(mut r)) => {
            if l.val < r.val {
                l.next = merge_two_lists(l.next, Some(r));
                Some(l)
            } else {
                r.next = merge_two_lists(Some(l), r.next);
                Some(r)
            }
        }
    }
}

///### [分隔链表](https://leetcode.cn/problems/partition-list/)
/// **关键字:** `拆为两条再合并为一条`
pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut smaller = ListNode::new(-1);
    let mut grater = ListNode::new(-1);
    let (mut p1, mut p2) = (&mut smaller, &mut grater);

    while let Some(mut node) = head {
        head = node.next.take();
        if node.val < x {
            p1.next = Some(node);
            p1 = p1.next.as_mut().unwrap();
        } else {
            p2.next = Some(node);
            p2 = p2.next.as_mut().unwrap();
        }
    }
    //此时分开完毕，进行再合并,此时p1指向smaller的尾节点，
    //要接上grater，则需要接上grater的next.
    p1.next = grater.next;
    smaller.next
}
/// ### [合并K个有序链表](https://leetcode.cn/problems/merge-k-sorted-lists/)
///  **关键字:** `新链表`、优先队列、对对象重新更改排序方式(已完成)
/// **简单描述:** 更改结构体的排序结构后，将每个链表加入到优先队列中依次取出，等队列为空时，就得到排好序的新链表了
/// 需要注意的是Rust的BinaryHeap默认为大根堆，所以需要对结构体更改一下排序规则。
pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    if lists.is_empty() {
        return None;
    }
    let mut heap = BinaryHeap::new();
    let mut dummy = ListNode::new(-1);
    let mut cur = &mut dummy;

    for list in lists {
        if let Some(node) = list {
            heap.push(node);
        }
    }
    while let Some(mut node) = heap.pop() {
        //再一次巩固，为啥需要take()，而不直接node.next
        //因为这样会把原本书属于node.next 所指向的值 的所有权 变为两个
        if let Some(item) = node.next.take() {
            heap.push(item)
        }
        cur.next = Some(node);
        cur = cur.next.as_mut().unwrap();
    }
    dummy.next
}
/// ### [删除链表倒数第n个节点](https://leetcode.cn/problems/remove-nth-node-from-end-of-list/)
/// **关键词:** 快慢指针、快指针先走n再同时走.
pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    if head.is_none() {
        return head;
    }
    let mut fast = head.clone();
    let slow = head;
    let mut dummy = ListNode::new(-1);
    dummy.next = slow;
    let mut cur = &mut dummy;
    //快指针先走n步
    for _ in 0..n {
        //有可能n > 链表长度，所以需要判断一下
        if let Some(node) = fast {
            fast = node.next;
        } else {
            return None;
        }
    }
    while fast.is_some() {
        fast = fast.unwrap().next;
        cur = cur.next.as_mut().unwrap();
    }

    //分析一遍为啥这么写，假设链表为 1 -> 2 -> 3 -> 4 -> 5. n = 2
    //此时，cur 指向 3，cur的类型为&mut ListNode
    // cur.next = 4,
    //如果直接 cur.next = cur.next.unwrap().next
    //会发生5既属于4又属于3
    //为啥4也需要take(),估计是设计问题，其实简单的cur.next = cur.next也会报错。
    cur.next = cur.next.take().unwrap().next.take();
    dummy.next
}

///###[链表的中间节点](https://leetcode.cn/problems/middle-of-the-linked-list/)
/// **关键词:**快慢指针,快指针每次走两步，慢指针每次走一步
pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }
    let mut fast = &head;
    let mut slow = &head;
    //看到一个大佬写的，简洁明了
    while fast.is_some() {
        fast = &fast.as_ref().unwrap().next;
        fast = &fast.as_ref().unwrap().next;
        slow = &slow.as_ref().unwrap().next;
    }
    //善用克隆，避免了一个虚拟头结点
    slow.clone()
}

///### [环形链表](https://leetcode.cn/problems/linked-list-cycle/)
/// 由于Rust中的所有权设计，所以不可能出现环形链表，所以直接看其他语言的版本
/// 简单来说，也是快慢指针，只要一个走得快走得慢，那么总有遇到的时候
pub fn has_cycle(_head: Option<Box<ListNode>>) -> bool {
    false
}

///### [相交链表](https://leetcode.cn/problems/intersection-of-two-linked-lists/)
/// 由于Rust中的所有权设计，所以不可能出现相交链表，所以直接看其他语言的版本
/// 简单来说，就是让他们走一样的路程，如果相交就会遇到。
pub fn get_intersection_node(
    _list1: Option<Box<ListNode>>,
    _list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    None
}

///### [反转链表](https://leetcode.cn/problems/reverse-linked-list/)
/// Rust用栈很简单，时间复杂度O(2n)
/// 如何用递归或一次遍历解决
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
///### 反转链表递归版
pub fn reverse_list_recursive(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn recursive(
        prev: Option<Box<ListNode>>,
        head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if let Some(mut node) = head {
            let tail = node.next.take();
            node.next = prev;
            return recursive(tail, Some(node));
        }
        prev
    }
    recursive(None, head)
}

pub fn reverse_list_n(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    if head.as_ref().is_none() || head.as_ref().unwrap().next.as_ref().is_none() {
        return head;
    }
    let mut k = k;
    let mut tail: Option<Box<ListNode>> = None;
    let mut new_head = None;

    while let Some(mut node) = head {
        if k == 0 {
            tail = Some(node);
            break;
        }
        head = node.next.take();
        node.next = new_head;
        new_head = Some(node);
        k -= 1;
    }

    let mut cur = &mut new_head;

    while cur.as_ref().unwrap().next.is_some() {
        cur = &mut cur.as_mut().unwrap().next;
    }

    cur.as_mut().unwrap().next = tail;
    new_head
}

pub fn reverse_list2(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    if left == 1 {
        return reverse_list_n(head, right);
    }
    head.as_mut().unwrap().next =
        reverse_list2(head.as_mut().unwrap().next.take(), left - 1, right - 1);
    head
}

pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut remain = head;
    let mut dummy = Box::new(ListNode::new(0));
    let mut tail = &mut dummy;
    while remain.is_some() {
        let (new_head, new_remain) = reverse_one(remain, k);
        remain = new_remain;
        tail.next = new_head;
        while tail.next.as_ref().is_some() {
            tail = tail.next.as_mut().unwrap();
        }
    }

    dummy.next
}

// 反转一次，返回反转后的head和remain
// 如果为最后一次不足以反转，remain为None
fn reverse_one(
    head: Option<Box<ListNode>>,
    k: i32,
) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
    let mut remain = head;
    let mut dummy = ListNode::new(0);
    for _ in 0..k {
        if let Some(mut n) = remain {
            remain = n.next.take();
            n.next = dummy.next.take();
            dummy.next = Some(n);
        }
    }

    (dummy.next, remain)
}

///###[回文链表](https://leetcode.cn/problems/palindrome-linked-list/)
/// **关键词:**数组，或者反转链表
pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let mut head = head;
    let mut vec = Vec::new();
    while let Some(mut node) = head {
        head = node.next.take();
        vec.push(node.val);
    }
    let len = vec.len();
    for i in 0..len / 2 {
        if vec[i] != vec[len - i - 1] {
            return false;
        }
    }
    true
}

///### 删除链表中的重复项(https://leetcode.cn/problems/remove-duplicates-from-sorted-list/)
/// 快慢指针
pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return head;
    }
    let mut head = head;
    let mut node = head.as_mut().unwrap();
    let mut duplicate = node.val;

    while let Some(next) = node.next.take() {
        // 先夺
        if next.val == duplicate {
            node.next = next.next;
        } else {
            duplicate = next.val; // 更新下次比较值
            node.next = Some(next); // 放回去
            node = node.next.as_mut().unwrap(); // 慢指针前进
        }
    }
    head
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::linkedlist;

    #[test]
    fn test_merge_two_lists() {
        // merge_two_lists(list1, list2)
        // partition(head, x)
        // let head = create_list();

        let head = linkedlist![1, 2, 3, 4];
        println!("{:?}", middle_node(head));
    }
}
