#![feature(mixed_integer_ops)]

use std::{cmp::Ordering, collections::HashMap, ptr};

pub struct Solution;

impl Solution {
    /// LCP 42 - Circle Game (772 ms 3.7 MB)
    ///
    /// toy (x, y, r), cycle (x, y). Simply traverse the toys smaller than
    /// circles for each circle and filter toys in circles then return the
    /// count after filter. A toy in a circle if their distance of centres
    /// isn't greater than their radius subtraction.
    #[rustfmt::skip]
    pub fn circle_game(
        toys: &Vec<(usize, usize, usize)>,
        circles: &Vec<(usize, usize)>,
        radius: usize,
    ) -> usize {
        toys.iter()
            .filter(|toy| radius.ge(&toy.2))
            .filter_map(|toy| {
                circles
                    .iter()
                    .skip_while(|circle| {
                        circle.0.abs_diff(toy.0).pow(2)
                            + circle.1.abs_diff(toy.1).pow(2)
                            > radius.abs_diff(toy.2).pow(2)
                    })
                    .next()
            })
            .count()
    }

    /// 599 - Minimum Index Sum of Two Lists (8 ms 2.3 MB)
    ///
    /// Suppose Andy and Doris want to choose a restaurant for dinner,
    /// and they both have a list of favorite restaurants represented
    /// by strings.
    ///
    /// You need to help them find out their common interest with the
    /// least list index sum. If there is a choice tie between answers,
    /// output all of them with no order requirement. You could assume
    /// there always exists an answer.
    ///
    /// Firstly, we entry the `list2` into a hash map. Then we traverse
    /// the `list1` and find the same keys from `list2`. We first guess
    /// the min index sum is `usize::MAX`, then if a new index sum less
    /// than old one, we clear the `result` and then push the `key` into
    /// it, or a index sum is same as the `min` we just push `key` into
    /// it but not clear the `result`.
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut result = Vec::new();
        let mut min = usize::MAX;
        let map = list2
            .into_iter()
            .enumerate()
            .map(|item| (item.1, item.0))
            .collect::<HashMap<_, _>>();
        list1.into_iter().enumerate().for_each(|(idx1, key)| {
            if let Some(idx2) = map.get(&key) {
                let idx = idx1 + idx2;
                match idx.cmp(&min) {
                    Ordering::Less => {
                        result.clear();
                        result.push(key);
                        min = idx;
                    }
                    Ordering::Equal => result.push(key),
                    _ => (),
                }
            }
        });
        result
    }

    #[allow(rustdoc::broken_intra_doc_links)]
    /// 798 - Smallest Rotation with Highest Score (4 ms 4 MB)
    ///
    /// You are given anarray nums. You can rotate it by a non-negative integer k so that the
    /// array becomes [nums[k], nums[k + 1], ... nums[nums.length - 1], nums[0], nums[1], ...,
    /// nums[k-1]]. Afterward, any entries that are less than or equal to their index are worth
    /// one point.
    ///
    /// For example, if we have nums = [2, 4, 1, 3, 0], and we rotate by k = 2, it becomes [1,
    /// 3, 0, 2, 4]. This is worth 3 points because 1 > 0 [no points], 3 > 1 [no points], 0 <=
    /// 2 [one point], 2 <= 3 [one point], 4 <= 4 [one point].
    ///
    /// Return the rotation index k that corresponds to the highest score we can achieve if we
    /// rotated nums by it. If there are multiple answers, return the smallest such index k.
    ///
    /// Constraints:
    ///
    /// - 1 <= nums.length <= 10^5
    /// - 0 <= nums[i] < nums.length
    ///
    /// Firstly, we map the diffs of each num, and calculate the score of `k = 0`. For `diff >
    /// 0`, we map the nums to it's last `k` it could get a point. Then we traverse `k` from 1
    /// to the end. There are many changes before caculate the next score. First step is get
    /// current diff (which will move to the end). If it has got a score, then minus a point
    /// previous (it won't lose a point after moving to the end), others add a point to it's
    /// first `k` that got a point. Note that a negative diff (the diff after apply the `k` is
    /// negative) will always get a point after a rotation. Then we minus the score will lost
    /// after a rotation. After changes, `k += 1` (a rotation has finished). If current score
    /// greater than max score, then update the `result`.
    pub fn best_rotation(nums: &Vec<usize>) -> usize {
        let diffs = nums
            .iter()
            .zip(0isize..)
            .map(|(&num, idx)| idx.wrapping_sub_unsigned(num))
            .collect::<Vec<_>>();
        let mut map = vec![0usize; nums.len()];
        let mut result = (
            0usize,
            diffs
                .iter()
                .filter(|diff| !diff.is_negative())
                .map(|diff| map[diff.unsigned_abs()] += 1)
                .count(),
        );
        let (mut idx, mut score) = result;
        diffs.iter().for_each(|&diff| {
            diff.wrapping_sub_unsigned(idx)
                .is_negative()
                .then(|| score += 1);
            match diff.is_negative() {
                true => map[nums.len().wrapping_add_signed(diff)] += 1,
                false => map[diff.unsigned_abs()] -= 1,
            }
            score -= map[idx];
            idx += 1;
            if score > result.1 {
                result = (idx, score);
            }
        });
        result.0
    }

    /// 1309 - Decrypt String from Alphabet to Integer Mapping (0 ms 2 MB)
    ///
    /// You are given a string `str` formed by digits and `'#'`. We want to map `str` to English
    /// lowercase characters as follows:
    ///
    /// - Characters (`'a'` to `'i'`) are represented by (`'1'` to `'9'`) respectively.
    /// - Characters (`'j'` to `'z'`) are represented by (`'10#'` to `'26#'`) respectively.
    ///
    /// Firstly, we create a queue which map to the high and the low. Then we traverse the `str`.
    /// When a byte is `'#'`, we pop both the high and the low as a char, others we push the byte
    /// into the queue and pop the high as a char. If the queue isn't empty after traversal, pop
    /// the remaining bytes. After all steps, we build a dist string successfully.
    pub fn freq_alphabets(str: &str) -> String {
        let mut state = [None; 2];
        let high = &mut state[0] as *mut Option<u8>;
        let low = &mut state[1] as *mut Option<u8>;
        String::from_iter(
            str.bytes()
                .into_iter()
                .filter_map(|c| match c {
                    b'#' => Some(unsafe {
                        let high = ptr::replace(high, None).unwrap();
                        let low = ptr::replace(low, None).unwrap();
                        b'a' - 1 + (high << 3) + (high << 1) + low
                    }),
                    _ => unsafe {
                        ptr::swap(high, low);
                        match ptr::replace(low, Some(c - b'0')) {
                            Some(c) => Some(b'a' - 1 + c),
                            None => None,
                        }
                    },
                })
                .chain(
                    state
                        .iter()
                        .filter(|c| c.is_some())
                        .map(|c| b'a' - 1 + c.unwrap()),
                )
                .map(|c| c as char),
        )
    }

    #[rustfmt::skip]
    /// 2055 - Plates Between Candles (48 ms 9.6 MB)
    ///
    /// There is a long table with a line of plates and candles arranged on top of it.
    /// You are given a 0-indexed string `str` consisting of characters `'*'` and `'|'`
    /// only, where a `'*'` represents a plate and a `'|'` represents a candle.
    ///
    /// You are also given a 0-indexed 2D integer array `queries` where `queries[i] =
    /// (left, right)` denotes the substring `str[left..=right]`. For each query, you
    /// need to find the number of plates between candles that are in the substring. A
    /// plate is considered between candles if there is at least one candle to its left
    /// and at least one candle to its right in the substring.
    ///
    /// For solving this problem, we can simply traverse`str` for each query, but it's
    /// sloooooowly (O(m * n)). We have to find an alternative. Firstly, we can get the
    /// left plates count of each index of str. Then we get `the closest candle index`
    /// both the `left side` and `right side` of each index of `str`. Finally we minus
    /// `the left plates count` of `the closest left candle index of query[right]` and
    /// `the left plates count` of `the closest right candle index of query[left]`, but
    /// we need to verify whether both `the closest left candle index of query[right]`
    /// and `the closest right candle index of query[left]` are valid. (which actually
    /// O(3 * m + n))
    pub fn plates_between_candles(
        str: &str,
        queries: &Vec<(usize, usize)>
    ) -> Vec<usize> {
        let mut cnt = 0;
        let map = str
            .bytes()
            .into_iter()
            .map(|c| {
                if c == b'*' {
                    cnt += 1;
                }
                cnt
            })
            .collect::<Vec<_>>();
        let mut idx = None;
        let left = str
            .bytes()
            .enumerate()
            .map(|(i, c)| {
                if c == b'|' {
                    idx = Some(i);
                }
                idx
            })
            .collect::<Vec<_>>();
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
