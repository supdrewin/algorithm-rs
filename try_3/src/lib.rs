pub mod btree;
pub mod treap;

use btree::BTreeNode;
use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    /// Question 1 - Find All Suspects
    ///
    /// Firstly, we init a disjoint set with the total size. Then we write
    /// a closure for finding a index. For getting the suspects count, the
    /// height of each index should be recorded. Next step we traverse each
    /// groups and union the students. The Question says student 0 always a
    /// suspect. So the result is the height if the branch of index 0.
    pub fn find_all_suspects(groups: &Vec<Vec<usize>>, total: usize) -> usize {
        assert!(total > 0);
        let mut set = (0..total).into_iter().collect::<Vec<usize>>();
        let set = set.as_mut_ptr();
        let find = |idx: usize| unsafe {
            let mut idx = idx;
            let mut next;
            while {
                next = *set.offset(idx as isize);
                idx != next
            } {
                idx = next;
            }
            idx
        };
        let mut height = vec![1; total];
        groups.iter().for_each(|students| {
            students
                .iter()
                .zip(students.iter().skip(1))
                .for_each(|(&idx1, &idx2)| unsafe {
                    let (idx1, idx2) = (find(idx1), find(idx2));
                    idx1.ne(&idx2).then(|| {
                        set.offset(idx1 as isize).write(idx2);
                        height[idx2] += 1;
                    });
                });
        });
        height[find(0)]
    }

    /// Question 2 - Postorder Traversal
    ///
    /// The first step we build a `BTree` from the `preorder` and `inorder`,
    /// then call the postorder method from the `BTreeNode`. For building a
    /// `BTree`, we traverse around the `preorder` then we get the index of
    /// each elements map to the `inorder` for split building the `left` and
    /// `right` tree node. If map index not found, the `None` matched, then
    /// we just build a empty node.
    #[rustfmt::skip]
    pub fn get_postorder_traversal<T: Clone + Eq>(
        preorder: &Vec<T>,
        inorder: &Vec<T>,
    ) -> Vec<T> {
        fn build<T: Clone + Eq>(
            preorder: &Vec<T>,
            inorder: &Vec<T>,
            left: usize,
            right: usize,
            cnt: &mut usize,
        ) -> Option<Box<BTreeNode<T, ()>>> {
            if let Some(idx) = inorder[left..right]
                .iter()
                .zip(left..right)
                .skip_while(|(val, _)| preorder[*cnt].ne(val))
                .map_while(|(_, idx)| Some(idx))
                .next()
            {
                *cnt += 1;
                BTreeNode::build(
                    BTreeNode::new(inorder[idx].clone(), ()),
                    build(preorder, inorder, left, idx, cnt),
                    build(preorder, inorder, idx + 1, right, cnt),
                )
            } else {
                None
            }
        }
        match build(preorder, inorder, 0, preorder.len(), &mut 0) {
            Some(btree) => btree
                .postorder()
                .map(|node| node.key.clone())
                .collect(),
            None => Vec::new(),
        }
    }

    /// Question 3 - hdu 4585 "Shaolin"
    ///
    /// This problem can be simply solved by `Binary Search Tree`.
    /// On this moment, I'm  lazy to write a `treap` (WIP). So we
    /// use `BTreeMap` from Standard Library instead. Firstly, We
    /// create a `BtreeMap` and insert the Master (Lv 10^9, ID 1).
    /// Then traversing each monk, we insert his (Lv, ID) and then
    /// get the closest monk's ID by compare their level.
    pub fn recover_lost_records(monks: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        let mut map = BTreeMap::new();
        map.insert(1_000_000_000, 1);
        monks
            .iter()
            .map(|&(id, lv)| {
                map.insert(lv, id);
                let prev = map.range(..lv).next_back();
                let next = map.range(lv + 1..).next();
                (
                    id,
                    *match prev {
                        Some(prev) => match next {
                            Some(next) => match prev.0 + next.0 < lv << 1 {
                                false => prev,
                                true => next,
                            },
                            None => prev,
                        },
                        None => next.unwrap(),
                    }
                    .1,
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests;
