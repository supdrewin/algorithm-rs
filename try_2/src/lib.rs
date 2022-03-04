use std::{collections::HashSet, mem};

pub struct Solution;

impl Solution {
    /// Question 1 - Pentagram Game
    ///
    /// Enter 1..=12 into pentagram, except 7 and 11. the spins
    /// and mirrors are trusted as same. The pentagram and each
    /// index are as follow.
    ///      0
    /// 4  7  8   1
    ///   6    9
    ///     5
    /// 3       2
    ///
    /// The number filled into should be:
    /// [0] + [7] + [6] == [5] + [9] + [1]
    /// [3] + [5] + [9] == [8] + [7] + [4]
    /// [1] + [8] + [7] == [6] + [5] + [2]
    /// [4] + [6] + [5] == [9] + [8] + [0]
    ///
    /// Note: this method using `DFS` is extremely sloooooowly
    /// and it's bound to time out.
    pub fn pentagram_game() -> Vec<Vec<i32>> {
        use try_1::Solution;
        let spin = |vec: &Vec<i32>| {
            let mut vec = vec.clone();
            for i in [0, 1, 2, 3, 5, 6, 7, 8] {
                vec.swap(i, i + 1);
            }
            vec
        };
        let mirror = |vec: &Vec<i32>| {
            let mut vec = vec.clone();
            vec.swap(1, 4);
            vec.swap(2, 3);
            vec.swap(6, 9);
            vec.swap(7, 8);
            vec
        };
        let mut result = Vec::new();
        let mut set = HashSet::new();
        for vec in &mut Solution::full_permutation(&vec![1, 2, 3, 4, 5, 6, 8, 9, 10, 12]) {
            if vec[0] + vec[7] + vec[6] == vec[5] + vec[9] + vec[1] {
                if vec[3] + vec[5] + vec[9] == vec[8] + vec[7] + vec[4] {
                    if vec[1] + vec[8] + vec[7] == vec[6] + vec[5] + vec[2] {
                        if vec[4] + vec[6] + vec[5] == vec[9] + vec[8] + vec[0] {
                            if !set.contains(vec) {
                                result.push(vec.clone());
                                for _ in 0..5 {
                                    set.insert(mem::replace(vec, spin(&vec)));
                                    set.insert(mirror(&vec));
                                }
                            }
                        }
                    }
                }
            }
        }
        result
    }

    /// Question 2 - Red and Black
    ///
    /// '.' is a black block, '#' is a red block, '@' is a human. The human can only
    /// move between the black blocks. This method use a simple `DFS` algorithm, but
    /// it's sloooooowly.
    pub fn red_and_black(room: &Vec<Vec<char>>, pos: (usize, usize)) -> usize {
        let mut result = 1;
        let mut goto = |pos: (usize, usize)| {
            let mut room = room.clone();
            if let Some(vec) = room.get_mut(pos.0) {
                if let Some(c) = vec.get_mut(pos.1) {
                    if mem::replace(c, '#') == '.' {
                        result = result.max(Self::red_and_black(&room, pos) + 1);
                    }
                }
            }
        };
        if pos.0 > 0 {
            goto((pos.0 - 1, pos.1));
        }
        if pos.1 > 0 {
            goto((pos.0, pos.1 - 1));
        }
        goto((pos.0 + 1, pos.1));
        goto((pos.0, pos.1 + 1));
        result
    }

    /// Question 3
    ///
    ///
    pub fn sliding_puzzle() {
        todo!()
    }

    /// Get the inverse number of a board.
    ///
    /// This method is mainly for verifying whether a board
    /// is valid.
    fn _inverse_number() {
        todo!()
    }

    /// Question 4
    ///
    /// Write an algorithm to print all ways of arranging n
    /// queens on an n x n chess board so that none of them
    /// share the same row, column, or diagonal. In this case,
    /// "diagonal" means all diagonals, not just the two that
    /// bisect the board.
    ///
    /// https://leetcode-cn.com/problems/eight-queens-lcci
    pub fn solve_n_queens() {
        todo!()
    }
}

#[cfg(test)]
mod tests;
