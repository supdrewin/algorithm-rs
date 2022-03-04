use std::mem;

pub struct Solution;

impl Solution {
    /// Question 1
    ///
    /// Enter 1..=12 into pentagram, except 7 and 11.
    pub fn pentagram_game() -> Vec<i32> {
        todo!()
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
