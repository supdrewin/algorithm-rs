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
                    (None, None) => panic!("No any matched old monks for this new monk!"),
                    (Some(prev), None) => prev,
                    (None, Some(next)) => next,
                    (Some(prev), Some(next)) => match prev.0 + next.0 < lv << 1 {
                        false => prev,
                        true => next,
                    },
                }
                .1
                .to_owned();
                map.insert(lv, id);
                (id, result)
            })
            .collect()
    }

    /// Question 2
    ///
    ///
    pub fn count_enemies(
        case: &mut Vec<usize>,
        commands: &Vec<(String, usize, usize)>,
    ) -> Vec<usize> {
        let mut result = Vec::new();
        commands
            .iter()
            .for_each(|command| match command.0.as_str() {
                "Query" => result.push(case[command.1 - 1..command.2].iter().sum()),
                "Add" => case[command.1 - 1] += command.2,
                "Sub" => case[command.1 - 1] -= command.2,
                _ => panic!("unknown command detected"),
            });
        result
    }

    /// Question 3
    ///
    /// (x1, x2), (x2, y2)
    /// x1 <= x2, y1 >= y2
    /// y1 - x1 > y2 - x2
    pub fn question_3(_input: &Vec<(usize, usize)>) -> Vec<usize> {
        todo!()
    }
}

#[cfg(test)]
mod tests;
