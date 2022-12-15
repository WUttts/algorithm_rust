use std::{cell::RefCell, rc::Rc};

use crate::mystruct::binarytree::TreeNode;

pub fn trarevse(root: Option<Rc<RefCell<TreeNode>>>) {
    match root {
        Some(node) => {
            println!("{:?}", &node.borrow().val);
            trarevse(node.borrow_mut().left.take());
            trarevse(node.borrow_mut().right.take());
        }
        None => return,
    }
}

pub fn max_depth(_root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test_traverse() {
        let root = tree![1, 2, 3, 4, 5, 6];
        trarevse(root);
    }
}
