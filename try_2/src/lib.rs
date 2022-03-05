use std::{
    collections::{HashSet, VecDeque},
    mem, ptr,
};

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

    /// Question 3 - Sliding Puzzle Problem
    ///
    /// TODO
    pub fn sliding_puzzle<T: PartialEq + Clone>(
        src: &Vec<Vec<T>>,
        dist: &Vec<Vec<T>>,
        nul: T,
    ) -> Option<usize> {
        let bound = (
            (0, src.len() - 1),
            (0, src.get(0).expect("Get bound failed!").len() - 1),
        );
        let mut pos = Default::default();
        for x in bound.0 .0..bound.0 .1 {
            for y in bound.1 .0..bound.1 .1 {
                if src[x][y] == nul {
                    pos = (x, y);
                    break;
                }
            }
        }
        let mut deque = VecDeque::new();
        deque.push_back((src.clone(), pos, 0));
        while !deque.is_empty() {
            let (src, pos, step) = deque.pop_front().unwrap();
            if *dist == src {
                return Some(step);
            }
            let mut direct = Vec::new();
            if pos.0 > bound.0 .0 {
                direct.push((pos.0 - 1, pos.1));
            }
            if pos.0 < bound.0 .1 {
                direct.push((pos.0 + 1, pos.1));
            }
            if pos.1 > bound.1 .0 {
                direct.push((pos.0, pos.1 - 1));
            }
            if pos.1 < bound.1 .1 {
                direct.push((pos.0, pos.1 + 1));
            }
            for next in direct {
                let mut src = src.clone();
                unsafe {
                    ptr::swap(
                        &mut src[next.0][next.1] as *mut T,
                        &mut src[pos.0][pos.1] as *mut T,
                    );
                }
                deque.push_back((src, next, step + 1));
            }
        }
        None
    }

    /// Get the inverse number of a board.
    ///
    /// This method is mainly for verifying whether a board
    /// is valid.
    fn _inverse_number() {
        todo!()
    }

    /// Question 4 - Eight Gueens Problem
    ///
    /// Write an algorithm to print all ways of arranging n queens
    /// on an n x n chess board so that none of them share the same
    /// row, column, or diagonal. In this case, "diagonal" means all
    /// diagonals, not just the two that bisect the board.
    ///
    /// https://leetcode-cn.com/problems/eight-queens-lcci
    ///
    /// This problem can be simply solved by `DFS`. Firstly, we build
    /// a "map" with each line to it's column, then we fork the "map"
    /// each different choose (column). The only thing we should pay
    /// attention is cut the fork invalid (line 162 ~ 169). When the
    /// recursion on the top (map forked is full), we convert the map
    /// to result's element (149 ~ 158).
    ///
    /// # Issues
    ///
    /// Add this if FromIterator didn't preluded by your compiler.
    ///
    /// ``` rust
    /// use core::iter::FromIterator;
    /// ```
    ///
    /// If you are using a `stable` Rust and have some issues with the
    /// `abs_diff`. Simply add this following function into the body,
    /// then modify the line 164.
    ///
    /// ``` rust
    /// use std::{cmp::Ordering, ops::Sub};
    ///
    /// fn abs_diff<T: Default + Ord + Sub<Output = T>>(a: T, b: T) -> T {
    ///     match a.cmp(&b) {
    ///         Ordering::Equal => T::default(),
    ///         Ordering::Greater => a - b,
    ///         Ordering::Less => b - a,
    ///     }
    /// }
    /// ```
    pub fn solve_n_queens(n: usize) -> Vec<Vec<String>> {
        fn search(map: Vec<usize>, x: usize, result: &mut Vec<Vec<String>>) {
            if x == map.len() {
                result.push(
                    vec![vec!['.'; map.len()]; map.len()]
                        .iter_mut()
                        .enumerate()
                        .map(|(x, vec)| {
                            vec[map[x]] = 'Q';
                            String::from_iter(vec.iter())
                        })
                        .collect(),
                );
            } else {
                for y in 0..map.len() {
                    if {
                        let mut check = true;
                        for i in 0..x {
                            if map[i] == y || map[i].abs_diff(y) == i.abs_diff(x) {
                                check = false;
                                break;
                            }
                        }
                        check
                    } {
                        let mut map = map.clone();
                        map[x] = y;
                        search(map, x + 1, result);
                    }
                }
            }
        }
        let mut result = Vec::new();
        search(vec![0usize; n], 0, &mut result);
        result
    }
}

#[cfg(test)]
mod tests;
