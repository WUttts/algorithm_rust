#![allow(unused)]

use std::collections::{BinaryHeap, LinkedList};

use algorithm_rust::mystruct::list::ListNode;

fn main() {
    let size = 4;
    let mut ans = vec![vec![0; size]; size];
    ans[0][1] = 3;
    ans[1][1] = 4;
    ans[2][1] = 5;
    ans[3][1] = 6;
    let b = ans
        .into_iter()
        .flatten()
        .filter(|x| *x != 0)
        .collect::<Vec<i32>>();
    println!("{:?}", b);
    println!("{:?}"," ".as_bytes());
}
