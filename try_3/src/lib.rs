pub struct Solution;

impl Solution {
    #[rustfmt::skip]
    pub fn plates_between_candles(
        str: &str,
        queries: &Vec<(usize, usize)>
    ) -> Vec<usize> {
        let mut cnt = 0;
        let map: Vec<usize> = str
            .bytes()
            .into_iter()
            .map(|c| {
                if c == b'*' {
                    cnt += 1;
                }
                cnt
            })
            .collect();
        let mut idx = None;
        let left: Vec<Option<usize>> = str
            .bytes()
            .enumerate()
            .map(|(i, c)| {
                if c == b'|' {
                    idx = Some(i);
                }
                idx
            })
            .collect();
        idx = None;
        let mut right = vec![None; str.len()];
        for (i, c) in str.bytes().enumerate().rev() {
            if c == b'|' {
                idx = Some(i);
            }
            right[i] = idx;
        }
        queries
            .iter()
            .map(|pos| {
                let (left, right) = (
                    match right[pos.0] {
                        Some(left) => left,
                        None => return 0,
                    },
                    match left[pos.1] {
                        Some(right) => right,
                        None => return 0,
                    },
                );
                match left < right {
                    true => map[right] - map[left],
                    false => 0,
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests;
