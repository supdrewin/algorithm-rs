#![feature(is_sorted)]

pub struct Solution;

impl Solution {
    /// Quick Sort Algorithm
    ///
    /// We compare between the first index and the last index. If not compared,
    /// swap the elements under indices and move a step. After a rotation, the
    /// first index should equal the last index. Then we sort the left part and
    /// the right part with same method. Noted that if the src too big it will
    /// overflow the stack.
    pub fn quick_sort<F, T>(src: &Vec<T>, cmp: F) -> Vec<T>
    where
        F: Copy + FnMut(&T, &T) -> bool,
        T: Clone,
    {
        fn sort<F, T>(dist: &mut Vec<T>, mut cmp: F, (left, right): (usize, usize))
        where
            F: Copy + FnMut(&T, &T) -> bool,
            T: Clone,
        {
            let (mut idx1, mut idx2) = (left, right);
            while idx1 < idx2 {
                while idx1 < idx2 && cmp(&dist[idx1], &dist[idx2]) {
                    idx1 += 1;
                }
                if idx1 < idx2 {
                    dist.swap(idx1, idx2);
                    idx2 -= 1;
                }
                while idx1 < idx2 && cmp(&dist[idx1], &dist[idx2]) {
                    idx2 -= 1;
                }
                if idx1 < idx2 {
                    dist.swap(idx1, idx2);
                    idx1 += 1;
                }
            }
            if left + 1 < idx2 {
                sort(dist, cmp, (left, idx2 - 1));
            }
            if idx1 + 1 < right {
                sort(dist, cmp, (idx1 + 1, right));
            }
        }
        let mut dist = src.clone();
        (src.len() > 0).then(|| sort(&mut dist, cmp, (0, src.len() - 1)));
        dist
    }

    /// Merge Sort Algorithm
    ///
    /// We split the origin to be sorted in two parts, and then
    /// split each part in two part, and so on until the length
    /// of each part isn't greater than 1. Then we take elements
    /// in place for merging.
    pub fn merge_sort<F, T>(src: &Vec<T>, cmp: F) -> Vec<T>
    where
        F: Copy + FnMut(&T, &T) -> bool,
        T: Clone,
    {
        fn sort<F, T>((part1, part2): (&[T], &[T]), mut cmp: F) -> Vec<T>
        where
            F: Copy + FnMut(&T, &T) -> bool,
            T: Clone,
        {
            let sort = |part: &[T]| {
                if part.len() > 1 {
                    sort(part.split_at(part.len() >> 1), cmp)
                } else {
                    part.iter().map(|val| val.clone()).collect::<Vec<T>>()
                }
            };
            let part1 = sort(part1);
            let part2 = sort(part2);
            let mut dist = Vec::with_capacity(part1.len() + part2.len());
            let mut take = |src: &Vec<T>, idx: &mut usize| {
                dist.push(src[*idx].clone());
                *idx += 1;
            };
            let (mut idx1, mut idx2) = (0, 0);
            while idx1 < part1.len() && idx2 < part2.len() {
                if cmp(&part1[idx1], &part2[idx2]) {
                    take(&part1, &mut idx1);
                } else {
                    take(&part2, &mut idx2);
                }
            }
            while idx1 < part1.len() {
                take(&part1, &mut idx1);
            }
            while idx2 < part2.len() {
                take(&part2, &mut idx2);
            }
            dist
        }
        sort(src.split_at(src.len() >> 1), cmp)
    }
}

#[cfg(test)]
mod tests;
