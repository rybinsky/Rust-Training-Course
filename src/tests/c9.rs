use std::rc::Rc;

use crate::tasks::c9_smart_pointers::{BinaryTreeNode, Package, SharedCounter};

#[test]
fn test_btn_sum() {
    let tree = BinaryTreeNode::with_children(
        1,
        BinaryTreeNode::new(2),
        BinaryTreeNode::with_children(3, BinaryTreeNode::new(4), BinaryTreeNode::new(5)),
    );

    assert_eq!(tree.sum(), 15);
}

#[test]
fn test_shared_counter() {
    let counter = Rc::new(SharedCounter::new());

    let counter_clone1 = counter.clone();
    let counter_clone2 = counter.clone();

    counter_clone1.increment();
    counter_clone2.increment();

    assert_eq!(counter.get(), 2);
}
