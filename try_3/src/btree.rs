use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
};

pub struct BTreeNode<T> {
    value: T,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
}

impl<T> BTreeNode<T> {
    pub fn new(value: T) -> Option<Box<Self>> {
        Some(Box::new(Self {
            value,
            left: None,
            right: None,
        }))
    }

    pub fn build(
        mut node: Option<Box<Self>>,
        left: Option<Box<Self>>,
        right: Option<Box<Self>>,
    ) -> Option<Box<Self>> {
        if let Some(node) = &mut node {
            node.set_left(left);
            node.set_right(right);
        }
        node
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn left(&self) -> Option<&Self> {
        match &self.left {
            Some(node) => Some(node),
            None => None,
        }
    }

    pub fn right(&self) -> Option<&Self> {
        match &self.right {
            Some(node) => Some(node),
            None => None,
        }
    }

    pub fn set_value(&mut self, value: T) {
        self.value = value;
    }

    pub fn set_left(&mut self, left: Option<Box<Self>>) {
        self.left = left;
    }

    pub fn set_right(&mut self, right: Option<Box<Self>>) {
        self.right = right;
    }

    pub fn preorder<F, B>(&self, mut function: F) -> Vec<B>
    where
        F: FnMut(&Self) -> B,
    {
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(self);
        while let Some(node) = queue.pop_back() {
            result.push(function(node));
            if let Some(right) = node.right() {
                queue.push_back(right);
            }
            if let Some(left) = node.left() {
                queue.push_back(left);
            }
        }
        result
    }

    pub fn inorder<F, B>(&self, mut function: F) -> Vec<B>
    where
        F: FnMut(&Self) -> B,
    {
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        let mut cur = Some(self);
        while match cur {
            Some(node) => {
                queue.push_back(node);
                cur = node.left();
                true
            }
            None => match queue.pop_back() {
                Some(node) => {
                    result.push(function(node));
                    cur = node.right();
                    true
                }
                None => false,
            },
        } {}
        result
    }

    pub fn postorder<F, B>(&self, mut function: F) -> Vec<B>
    where
        F: FnMut(&Self) -> B,
    {
        let mut result = Vec::new();
        let mut queue1 = VecDeque::new();
        let mut queue2 = VecDeque::new();
        queue1.push_back(self);
        while let Some(node) = queue1.pop_back() {
            queue2.push_back(node);
            if let Some(left) = node.left() {
                queue1.push_back(left);
            }
            if let Some(right) = node.right() {
                queue1.push_back(right);
            }
        }
        while let Some(node) = queue2.pop_back() {
            result.push(function(node));
        }
        result
    }
}

impl<T> Deref for BTreeNode<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for BTreeNode<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

#[test]
#[rustfmt::skip]
fn test() {
    let btree = BTreeNode::build(
        BTreeNode::new(1),
        BTreeNode::build(
            BTreeNode::new(2),
            BTreeNode::build(
                BTreeNode::new(4),
                None,
                BTreeNode::new(7),
            ),
            None,
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
    )
    .unwrap();
    assert_eq!(
        btree.preorder(|node| **node),
        vec![1, 2, 4, 7, 3, 5, 8, 9, 6]
    );
    assert_eq!(
        btree.inorder(|node| **node),
        vec![4, 7, 2, 1, 8, 5, 9, 3, 6]
    );
    assert_eq!(
        btree.postorder(|node| **node),
        vec![7, 4, 2, 8, 9, 5, 6, 3, 1]
    );
}
