pub struct Solution;

impl Solution {
    #[rustfmt::skip]
    /// 2055 - Plates Between Candles (48 ms 9.6 MB)
    ///
    /// There is `a long table` with `a line of plates and candles`
    /// arranged on top of it. You are given a `0-indexed string`
    /// `str` consisting of characters b'*' and b'|' only, where a
    /// b'*' represents a plate and a b'|' represents a candle.
    ///
    /// You are also given a `0-indexed 2D integer array queries`
    /// where **queries[i] = (left, right)** denotes the substring
    /// `str[left..=right]`. For each query, you need to find the
    /// number of plates between candles that are in the substring.
    /// A plate is considered between candles if there is at least
    /// one candle to its left and at least one candle to its right
    /// in the substring.
    ///
    /// For solving this problem, we can simply match str for each
    /// query, but it's sloooooowly (O(m * n)). We have to find an
    /// alternative. Firstly, we can get the left plates count of
    /// each index of str. Then we get `the closest candle index`
    /// both the **left side** and **right side** of each index of
    /// `str`. Finally we minus `the **left** plates count` of `the
    /// closest **left** candle index of query[right]` and `the **
    /// left** plates count` of `the closest **right** candle index
    /// of query[left]`, but we also need to verify whether both `
    /// the closest **left** candle index of query[right]` and `the
    /// closest **right** candle index of query[left]` are valid.
    /// (actually O(3 * m + n))
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
