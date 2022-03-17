use treap::TreapMap;

pub struct Solution;

impl Solution {
    /// Question 1 - hdu 4585 "Shaolin"
    ///
    /// Same with Question 3 in `try_3`, but implements with my own
    /// `treap`. Feel free to test the `treap` for digging any BUGs
    /// and BUG reports are approved. **NOTED** that my `treap` is
    /// written for providing the APIs as similar as possiable with
    /// the Standard Library's `BTreeMap`, so the Docs of `TreapMap`
    /// is actually 1:1 copy from the Standard Library with smallest
    /// changes in order to test the compatibility between `TreapMap`
    /// and `BTreeMap`.
    pub fn recover_lost_records(monks: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        let mut map = TreapMap::new();
        map.insert(1_000_000_000, 1);
        monks
            .iter()
            .map(|&(id, lv)| {
                let result = match (map.range(..lv).next_back(), map.range(lv..).next()) {
                    (Some(prev), Some(next)) => match prev.0 + next.0 < lv << 1 {
                        false => prev,
                        true => next,
                    },
                    (Some(prev), None) => prev,
                    (None, Some(next)) => next,
                    (None, None) => panic!("No any matched old monks for this new monk!"),
                }
                .1
                .to_owned();
                map.insert(lv, id);
                (id, result)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests;
