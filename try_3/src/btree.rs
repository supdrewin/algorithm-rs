use std::collections::VecDeque;

pub struct BTreeNode<T> {
    pub value: T,
    pub left: Option<Box<Self>>,
    pub right: Option<Box<Self>>,
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
            node.left = left;
            node.right = right;
        }
        node
    }

    pub fn preorder(&self) -> PreOrder<T> {
        PreOrder::new(self)
    }

    pub fn inorder(&self) -> InOrder<T> {
        InOrder::new(self)
    }

    pub fn postorder(&self) -> PostOrder<T> {
        PostOrder::new(self)
    }
}

pub struct PreOrder<'a, T> {
    stack: VecDeque<Option<&'a BTreeNode<T>>>,
}

impl<'a, T> PreOrder<'a, T> {
    pub fn new(node: &'a BTreeNode<T>) -> Self {
        let mut stack = VecDeque::new();
        stack.push_back(Some(node));
        Self { stack }
    }
}

impl<'a, T> Iterator for PreOrder<'a, T> {
    type Item = &'a BTreeNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(Some(node)) = self.stack.pop_back() {
            if let Some(node) = &node.right {
                self.stack.push_back(Some(node));
            }
            if let Some(node) = &node.left {
                self.stack.push_back(Some(node));
            }
            self.stack.push_back(Some(node));
            self.stack.push_back(None);
        }
        match self.stack.pop_back() {
            Some(node) => node,
            None => None,
        }
    }
}

pub struct InOrder<'a, T> {
    stack: VecDeque<Option<&'a BTreeNode<T>>>,
}

impl<'a, T> InOrder<'a, T> {
    pub fn new(node: &'a BTreeNode<T>) -> Self {
        let mut stack = VecDeque::new();
        stack.push_back(Some(node));
        Self { stack }
    }
}

impl<'a, T> Iterator for InOrder<'a, T> {
    type Item = &'a BTreeNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(Some(node)) = self.stack.pop_back() {
            if let Some(node) = &node.right {
                self.stack.push_back(Some(node));
            }
            self.stack.push_back(Some(node));
            self.stack.push_back(None);
            if let Some(node) = &node.left {
                self.stack.push_back(Some(node));
            }
        }
        match self.stack.pop_back() {
            Some(node) => node,
            None => None,
        }
    }
}

pub struct PostOrder<'a, T> {
    stack: VecDeque<Option<&'a BTreeNode<T>>>,
}

impl<'a, T> PostOrder<'a, T> {
    pub fn new(node: &'a BTreeNode<T>) -> Self {
        let mut stack = VecDeque::new();
        stack.push_back(Some(node));
        Self { stack }
    }
}

impl<'a, T> Iterator for PostOrder<'a, T> {
    type Item = &'a BTreeNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(Some(node)) = self.stack.pop_back() {
            self.stack.push_back(Some(node));
            self.stack.push_back(None);
            if let Some(node) = &node.right {
                self.stack.push_back(Some(node));
            }
            if let Some(node) = &node.left {
                self.stack.push_back(Some(node));
            }
        }
        match self.stack.pop_back() {
            Some(node) => node,
            None => None,
        }
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
        btree.preorder().map(|node| node.value).collect::<Vec<_>>(),
        vec![1, 2, 4, 7, 3, 5, 8, 9, 6]
    );
    assert_eq!(
        btree.inorder().map(|node| node.value).collect::<Vec<_>>(),
        vec![4, 7, 2, 1, 8, 5, 9, 3, 6]
    );
    assert_eq!(
        btree.postorder().map(|node| node.value).collect::<Vec<_>>(),
        vec![7, 4, 2, 8, 9, 5, 6, 3, 1]
    );
}
