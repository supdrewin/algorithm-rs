use std::mem;

pub struct Solution;

impl Solution {
    /// Quick Sort Algorithm
    ///
    /// TODO: Add comments.
    pub fn quick_sort<F, T>(vec: &mut Vec<T>, cmp: F)
    where
        F: Copy + FnMut(&T, &T) -> bool,
    {
        fn sort<F, T>(vec: &mut Vec<T>, mut cmp: F, (left, right): (usize, usize))
        where
            F: Copy + FnMut(&T, &T) -> bool,
        {
            let (mut idx1, mut idx2) = (left, right);
            while idx1 < idx2 {
                while idx1 < idx2 && cmp(&vec[idx1], &vec[idx2]) {
                    idx1 += 1;
                }
                if idx1 < idx2 {
                    vec.swap(idx1, idx2);
                    idx2 -= 1;
                }
                while idx1 < idx2 && cmp(&vec[idx1], &vec[idx2]) {
                    idx2 -= 1;
                }
                if idx1 < idx2 {
                    vec.swap(idx1, idx2);
                    idx1 += 1;
                }
            }
            if left + 1 < idx2 {
                sort(vec, cmp, (left, idx2 - 1));
            }
            if idx1 + 1 < right {
                sort(vec, cmp, (idx1 + 1, right));
            }
        }
        assert!(vec.len() > 0);
        sort(vec, cmp, (0, vec.len() - 1));
    }

    /// Merge Sort Algorithm
    ///
    /// TODO: Add comments.
    pub fn merge_sort<F, T>(vec: &mut Vec<T>, cmp: F)
    where
        F: Copy + FnMut(&T, &T) -> bool,
        T: Clone,
    {
        fn sort<F, T>((slice1, slice2): (&[T], &[T]), mut cmp: F) -> Vec<T>
        where
            F: Copy + FnMut(&T, &T) -> bool,
            T: Clone,
        {
            let sort = |slice: &[T]| {
                if slice.len() > 1 {
                    sort(slice.split_at(slice.len() >> 1), cmp)
                } else {
                    slice.iter().map(|val| val.clone()).collect::<Vec<T>>()
                }
            };
            let vec1 = sort(slice1);
            let vec2 = sort(slice2);
            let mut dist = Vec::new();
            let mut take = |src: &Vec<T>, idx: &mut usize| {
                dist.push(src[*idx].clone());
                *idx += 1;
            };
            let (mut idx1, mut idx2) = Default::default();
            while idx1 < vec1.len() && idx2 < vec2.len() {
                if cmp(&vec1[idx1], &vec2[idx2]) {
                    take(&vec1, &mut idx1);
                } else {
                    take(&vec2, &mut idx2);
                }
            }
            let mut cnt = |vec: &Vec<T>, idx: &mut usize| {
                while *idx < vec.len() {
                    take(vec, idx);
                }
            };
            cnt(&vec1, &mut idx1);
            cnt(&vec2, &mut idx2);
            dist
        }
        mem::drop(mem::replace(vec, sort(vec.split_at(vec.len() >> 1), cmp)));
    }
}

#[cfg(test)]
mod tests;
