use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    /// Question 1
    ///
    /// Return
    ///
    /// vector with input number of people,
    /// true if bad people, otherwise good people.
    ///
    /// Params
    ///
    /// `input`, 2n, should always the power of 2
    /// m, pop the m'th people
    pub fn josephs_ring(input: i32, m: i32) -> Vec<bool> {
        let mut result = Vec::with_capacity(input as usize);
        for _ in 0..input {
            result.push(false);
        }
        let mut i = -1;
        for _ in 0..(input >> 1) {
            let mut m = m;
            while m > 0 {
                i += 1;
                if result[(i % input) as usize] != true {
                    m -= 1;
                }
            }
            result[(i % input) as usize] = true;
        }
        result
    }

    /// Question 2
    ///
    /// Judge if a sentence is palindrome.
    ///
    /// Firstly, we filter all alphabetic in the sentence and convert
    /// them as lowercase into a deque.
    /// Then we match the front and back value. If they aren't equal,
    /// the sentence is not palindrome. Otherwise, pop the previous
    /// values and match the next front and back.
    pub fn is_palindrome(sentence: &str) -> bool {
        let mut sentence: VecDeque<u8> = sentence
            .as_bytes()
            .iter()
            .filter(|c| c.is_ascii_alphabetic())
            .map(|c| return c.to_ascii_lowercase())
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

    /// Question 3
    ///
    /// recursive
    ///
    /// Big one should below small one.
    pub fn tower_of_hanoi(src: &mut Vec<i32>, tmp: &mut Vec<i32>, dist: &mut Vec<i32>) {
        Solution::_move(src, tmp, dist, src.len());
    }

    /// _move
    ///
    /// Privite method for tower_of_hanoi
    fn _move(src: &mut Vec<i32>, tmp: &mut Vec<i32>, dist: &mut Vec<i32>, counter: usize) {
        if counter > 0 {
            // Doubt: Why should we do (src -> tmp -> dist)?
            Solution::_move(src, dist, tmp, counter - 1);
            dist.push(src.pop().unwrap());
            Solution::_move(tmp, src, dist, counter - 1);
        }
    }

    /// Question 4
    ///
    /// recursive
    pub fn full_permutation(input: &Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        unsafe {
            Solution::_full_permutation(input.clone(), 0, &mut result);
        }
        result
    }

    /// _full_permutation
    ///
    /// Private method for recursion, noted that this method is extremely unsafe because
    /// `input` will move here and be push into result later. The initialize depth should
    /// always be 0.
    unsafe fn _full_permutation(input: Vec<i32>, depth: usize, result: &mut Vec<Vec<i32>>) {
        if depth == input.len() {
            result.push(input);
        } else {
            for i in depth..input.len() {
                let mut input = input.clone();
                input.swap(i, depth);
                Solution::_full_permutation(input, depth + 1, result);
            }
        }
    }
}

#[cfg(test)]
mod tests;
