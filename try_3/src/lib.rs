pub mod btree;

use btree::BTreeNode;

pub struct Solution;

impl Solution {
    /// Question 1 - Find All Suspects
    ///
    /// Firstly, we init a disjoint set with the total size. Then we write
    /// a closure for finding a index. For getting the suspects count, the
    /// height of each index should be recorded. Next step we traverse each
    /// groups and union the students. The Question says student 0 always a
    /// suspect. So the result is the height if the branch of index 0.
    #[rustfmt::skip]
    pub fn find_all_suspects(
        groups: &Vec<Vec<usize>>,
        total: usize)
    -> usize {
        assert!(total > 0);
        let mut set = (0..total)
            .into_iter()
            .collect::<Vec<usize>>();
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
        groups
            .iter()
            .map(|students| {
                students
                    .iter()
                    .zip(students.iter().skip(1))
                    .map(|(&idx1, &idx2)| unsafe {
                        let (idx1, idx2) = (find(idx1), find(idx2));
                        idx1.ne(&idx2).then(|| {
                            set.offset(idx1 as isize).write(idx2);
                            height[idx2] += 1;
                        });
                    })
                    .count();
            })
            .count();
        height[find(0)]
    }

    /// Question 2
    ///
    ///
    #[rustfmt::skip]
    pub fn get_postorder_traversal<T: Clone + Eq>(
        preorder: &Vec<T>,
        inorder: &Vec<T>)
    -> Vec<T> {
        fn build<T: Clone + Eq>(
            preorder: &Vec<T>,
            inorder: &Vec<T>,
            left: usize,
            right: usize,
            cnt: &mut usize,
        ) -> Box<Option<BTreeNode<T>>> {
            if let Some(idx) = inorder[left..right]
                .iter()
                .zip(left..right)
                .skip_while(|(val, _)| preorder[*cnt].ne(val))
                .map_while(|(_, idx)| Some(idx))
                .next()
            {
                *cnt += 1;
                BTreeNode::build(
                    BTreeNode::new(inorder[idx].clone()),
                    build(preorder, inorder, left, idx, cnt),
                    build(preorder, inorder, idx + 1, right, cnt),
                )
            } else {
                Box::new(None)
            }
        }
        build(preorder, inorder, 0, preorder.len(), &mut 0)
            .unwrap()
            .postorder(|node| node.value().clone())
    }

    /// Question 3
    ///
    ///
    pub fn recover_lost_records(monks: Vec<(usize, usize)>) {
        todo!()
    }
}

#[cfg(test)]
mod tests;
