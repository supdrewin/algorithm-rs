use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    /// Question 1 - Josephs Ring Problem.
    ///
    /// There are **2n** people, then pop a people per **m** people. It
    /// should left all good people after pop **n** people. Firstly, We
    /// guess all people as good people, then setting a pointer `idx`,
    /// which point to first people after apply an add. Then pop a bad
    /// people per **m** people. We only minus **m** if a good people,
    /// this mean the popped bad people are skipped. Finally, return a
    /// slice contains bad people with false and good people with true.
    pub fn josephs_ring(n: i32, m: i32) -> Vec<bool> {
        let len = (n as usize) << 1;
        let mut result = vec![true; len];
        let mut idx = -1;
        for _ in 0..n {
            let mut m = m;
            while m > 0 {
                idx += 1;
                if result[idx as usize % len] {
                    m -= 1;
                }
            }
            result[idx as usize % len] = false;
        }
        result
    }

    /// Question 2 - Judge if a sentence is palindrome.
    ///
    /// Firstly, we filter all **alphabet** and **number** in the
    /// sentence and convert them as **lowercase** into a `deque`.
    /// Then we match the front and back value. If they aren't equal,
    /// the sentence is not palindrome. Otherwise, pop the previous
    /// values and match the next front and back.
    pub fn is_palindrome(sentence: &str) -> bool {
        let mut sentence: VecDeque<u8> = sentence
            .as_bytes()
            .iter()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        while sentence.len() > 1 {
            if let Some(first) = sentence.front() {
                match sentence.back() {
                    Some(last) if first == last => {
                        sentence.pop_front();
                        sentence.pop_back();
                    }
                    _ => {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// Question 3 - Tower of Hanoi.
    ///
    /// The big one should always below small one and we can only move one. To
    /// solve this problem, the last one should be place to the `dist`,  so we
    /// firstly move all above to the `tmp`. After the last one in place, we
    /// move `tmp` to `dist`. And ..., the above one also should be placed to
    /// `dist`, then we do same thing with last one to the above one. Finally,
    /// we solve this problem by recursion.
    pub fn tower_of_hanoi(src: &mut Vec<i32>, tmp: &mut Vec<i32>, dist: &mut Vec<i32>) {
        fn mov(src: &mut Vec<i32>, tmp: &mut Vec<i32>, dist: &mut Vec<i32>, cnt: usize) {
            if cnt > 0 {
                mov(src, dist, tmp, cnt - 1);
                dist.push(src.pop().unwrap());
                mov(tmp, src, dist, cnt - 1);
            }
        }
        mov(src, tmp, dist, src.len());
    }

    /// Question 4 - Full Permutation
    ///
    /// We use a recursion to solve this problem, this method is also known as
    /// `DFS` _(usually Depth-First Search)_. This method **IS NOT** the direct
    /// implementation of full permutation, but implement the permutation `A(n,
    /// m)`. For the **Full Permutation** (a.k.a. `A(n, n)`)), just let `cnt =
    /// vec.len()`. The tuple vector has two part: The first part is the result
    /// of permutation, and the second part is the remaining elements of `vec`.
    /// We don't need to use `HashMap` (std) here because the implementation of
    /// `HashMap` (std) is sloooooowly.
    pub fn permutation<T: Clone>(vec: &Vec<T>, cnt: usize) -> Vec<(Vec<T>, Vec<T>)> {
        fn search<T: Clone>(
            mut vec: Vec<T>,
            cnt: usize,
            depth: usize,
            result: &mut Vec<(Vec<T>, Vec<T>)>,
        ) {
            if depth == cnt {
                let tail = vec.split_off(cnt);
                result.push((vec, tail));
            } else {
                for idx in depth..vec.len() {
                    let mut vec = vec.clone();
                    vec.swap(idx, depth);
                    search(vec, cnt, depth + 1, result);
                }
            }
        }
        assert!(cnt <= vec.len());
        let mut result = Vec::new();
        search(vec.clone(), cnt, 0, &mut result);
        result
    }

    /// Question 4 - Full Permutation
    ///
    /// This is a wrapper of previous permutation algorithm for
    /// directly get the full permutation from a vector.
    pub fn full_permutation<T: Clone>(vec: &Vec<T>) -> Vec<Vec<T>> {
        Self::permutation(vec, vec.len())
            .into_iter()
            .map(|v| v.0)
            .collect()
    }
}

#[cfg(test)]
mod tests;
