use std::cmp::Ordering;
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

    /// Question 2 - Count enemies
    ///
    /// The simplest way to solve this problem is processes each command
    /// and then executes them.
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

    /// Question 3 - Cows eat clover
    ///
    /// For `cow1` and `cow2`, the clover range their eat are `(x1, x2)`,
    /// `(x2, y2)`. If `cow1` stronger then `cow2`, it should satisfy `x1
    /// <= x2`, `y1 >= y2` and `y1 - x1 > y2 - x2`. This question asks us
    /// about how many cows stronger then each cow the given. Firstly, we
    /// process range `(x, y)` with the cows index to `vec`. Then we sort
    /// `vec` by `x1 < x2 or (x1 == x2 and y2 <= y1)` and then we map them
    /// with index before sorted -> index after sorted. After that we sort
    /// `vec` again by `y2 < y1 or (y1 == y2 and x1 <= x2)`. Finally, The
    /// stronger count of each cow is the minimal index between two sorted
    /// index of the cow.
    pub fn cows_eat_clover(range: &Vec<(usize, usize)>) -> Vec<usize> {
        let mut vec = range
            .iter()
            .enumerate()
            .map(|(cow, (x, y))| ((x, y), cow))
            .collect::<Vec<_>>();
        vec.sort_by(|((x1, y1), _), ((x2, y2), _)| match x1.cmp(x2) {
            Ordering::Equal => y2.cmp(y1),
            others => others,
        });
        let mut result = vec![0; range.len()];
        vec.iter()
            .enumerate()
            .for_each(|(idx, &(_, cow))| result[cow] = idx);
        vec.sort_by(|((x1, y1), _), ((x2, y2), _)| match y2.cmp(y1) {
            Ordering::Equal => x1.cmp(x2),
            others => others,
        });
        #[rustfmt::skip]
        vec.into_iter()
            .enumerate()
            .for_each(|(idx, (_, cow))| if idx < result[cow] {
                    result[cow] = idx;
            });
        result
    }
}

#[cfg(test)]
mod tests;
