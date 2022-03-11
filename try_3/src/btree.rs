use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
};

pub struct BTreeNode<T> {
    value: T,
    left: Box<Option<BTreeNode<T>>>,
    right: Box<Option<BTreeNode<T>>>,
}

impl<T> BTreeNode<T> {
    pub fn new(value: T) -> Box<Option<BTreeNode<T>>> {
        Box::new(Some(BTreeNode {
            value,
            left: Box::new(None),
            right: Box::new(None),
        }))
    }

    pub fn build(
        mut node: Box<Option<BTreeNode<T>>>,
        left: Box<Option<BTreeNode<T>>>,
        right: Box<Option<BTreeNode<T>>>,
    ) -> Box<Option<BTreeNode<T>>> {
        if let Some(node) = node.as_mut() {
            node.set_left(left);
            node.set_right(right);
        }
        node
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn left(&self) -> Option<&BTreeNode<T>> {
        match self.left.as_ref() {
            Some(node) => Some(node),
            None => None,
        }
    }

    pub fn right(&self) -> Option<&BTreeNode<T>> {
        match self.right.as_ref() {
            Some(node) => Some(node),
            None => None,
        }
    }

    pub fn set_value(&mut self, value: T) {
        self.value = value;
    }

    pub fn set_left(&mut self, left: Box<Option<BTreeNode<T>>>) {
        self.left = left;
    }

    pub fn set_right(&mut self, right: Box<Option<BTreeNode<T>>>) {
        self.right = right;
    }

    pub fn preorder<F, B>(&self, mut function: F) -> Vec<B>
    where
        F: FnMut(&BTreeNode<T>) -> B,
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
        F: FnMut(&BTreeNode<T>) -> B,
    {
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        let mut cur = Some(self);
        while {
            match cur {
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
            }
        } {}
        result
    }

    pub fn postorder<F, B>(&self, mut function: F) -> Vec<B>
    where
        F: FnMut(&BTreeNode<T>) -> B,
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
