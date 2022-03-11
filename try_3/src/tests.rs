use crate::{BTreeNode, Solution};

#[test]
fn find_all_suspects_1() {
    assert_eq!(
        Solution::find_all_suspects(
            &vec![
                vec![1, 2],
                vec![10, 13, 11, 12, 14],
                vec![0, 1],
                vec![99, 2],
            ],
            100
        ),
        4
    );
}

#[test]
#[rustfmt::skip]
fn find_all_suspects_2(){
    assert_eq!(
        Solution::find_all_suspects(
            &vec![
                vec![5],
                vec![1,2,3,4,5],
            ],
            200
        ),
        1
    );
}

#[test]
fn find_all_suspects_3() {
    assert_eq!(Solution::find_all_suspects(&vec![], 1), 1);
}

#[test]
fn get_postorder_traversal() {
    assert_eq!(
        Solution::get_postorder_traversal(
            &vec![1, 2, 4, 7, 3, 5, 8, 9, 6],
            &vec![4, 7, 2, 1, 8, 5, 9, 3, 6],
        ),
        vec![7, 4, 2, 8, 9, 5, 6, 3, 1]
    );
}

#[test]
#[rustfmt::skip]
fn node() {
    let tree = BTreeNode::build(
        BTreeNode::new(1),
        BTreeNode::build(
            BTreeNode::new(2),
            BTreeNode::build(
                BTreeNode::new(4),
                Box::new(None),
                BTreeNode::new(7),
            ),
            Box::new(None),
        ),
        BTreeNode::build(
            BTreeNode::new(3),
            BTreeNode::build(
                BTreeNode::new(5),
                BTreeNode::new(8),
                BTreeNode::new(9),
            ),
            BTreeNode::new(6),
        ),
    );
    if let Some(root) = tree.as_ref() {
        assert_eq!(
            root.preorder(|node| *node.value()),
            vec![1, 2, 4, 7, 3, 5, 8, 9, 6]
        );
        assert_eq!(
            root.inorder(|node| *node.value()),
            vec![4, 7, 2, 1, 8, 5, 9, 3, 6]
        );
        assert_eq!(
            root.postorder(|node| *node.value()),
            vec![7, 4, 2, 8, 9, 5, 6, 3, 1]
        );
    }
}
