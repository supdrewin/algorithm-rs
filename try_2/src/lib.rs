use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    mem, ptr,
};

pub struct Solution;

impl Solution {
    /// Question 1 - Pentagram Game
    ///
    /// Enter 1..=12 into pentagram, except 7 and 11. the spins and mirrors
    /// are trusted as same. The pentagram and each index are as follow.
    ///
    /// _____0___________________4(0)              <br />
    /// 8__1__7___6_____2(1)___4(1)_2(0)______3(2) <br />
    /// __2____5_____________4(2)_____3(1)         <br />
    /// ____4____________________3(0)              <br />
    /// 3_______9________4(3)____________1(0)      <br />
    ///
    /// The number filled into should be:
    ///
    /// 0 (4(0)) + 1 (4(1)) + 2 (4(2)) == 4 (3(0)) + 5 (3(1)) + 6 (3(2)) <br />
    /// 3 (4(3)) + 4 (3(0)) + 5 (3(1)) == 7 (2(0)) + 1 (4(1)) + 8 (2(1)) <br />
    /// 6 (3(2)) + 7 (2(0)) + 1 (4(1)) == 2 (4(2)) + 4 (3(0)) + 9 (1(0)) <br />
    /// 8 (2(1)) + 2 (4(2)) + 4 (3(0)) == 5 (3(1)) + 7 (2(0)) + 0 (4(0)) <br />
    ///
    /// Note: This method using `DFS` is extremely sloooooowly, but we can
    /// filter out some branches for hacking. As a result, 3.55s -> 0.07s.
    pub fn pentagram_game() -> Vec<Vec<i32>> {
        use try_1::Solution;
        let spin = |vec: &Vec<i32>| {
            let mut vec = vec.clone();
            vec.swap(0, 6);
            vec.swap(6, 9);
            vec.swap(9, 3);
            vec.swap(3, 8);
            vec.swap(4, 2);
            vec.swap(2, 1);
            vec.swap(1, 7);
            vec.swap(7, 5);
            vec
        };
        let mirror = |vec: &Vec<i32>| {
            let mut vec = vec.clone();
            vec.swap(1, 7);
            vec.swap(2, 5);
            vec.swap(3, 9);
            vec.swap(8, 6);
            vec
        };
        let mut result = Vec::new();
        let mut set = HashSet::new();
        let remain = vec![1, 2, 3, 4, 5, 6, 8, 9, 10, 12];
        for (vec4, remain) in Solution::permutation(&remain, 4) {
            let sum = vec4[0] + vec4[1] + vec4[2];
            let tmp1 = vec4[3] - vec4[1];
            for (vec3, remain) in Solution::permutation(&remain, 3) {
                let tmp2 = vec3[0] + vec3[1];
                if sum == tmp2 + vec3[2] {
                    let tmp = tmp1 + tmp2;
                    for (vec2, vec1) in Solution::permutation(&remain, 2) {
                        if tmp == vec2[0] + vec2[1] {
                            let tmp = vec4[2] + vec3[0] - vec2[0];
                            if tmp == vec4[0] + vec3[1] - vec2[1]
                                && tmp == vec4[1] + vec3[2] - vec1[0]
                            {
                                let mut vec = vec![
                                    vec4[0], vec4[1], vec4[2], vec4[3], vec3[0], vec3[1], vec3[2],
                                    vec2[0], vec2[1], vec1[0],
                                ];
                                if !set.contains(&vec) {
                                    result.push(vec.clone());
                                    for _ in 0..5 {
                                        let spin = spin(&vec);
                                        set.insert(mem::replace(&mut vec, spin));
                                        set.insert(mirror(&vec));
                                    }
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
    /// This method using a Two-way `BFS` which prefered to search
    /// the shortest path when both the `src` and `dist` are known.
    /// Firstly, we check the bound of the board, and then search
    /// the `null slot` to be swapped. Then we need to calculate a
    /// _"magical number"_ (a.k.a. `inverse number`) for verifying
    /// whether the board is valid. The detail is that we have to
    /// map the `src` -> `dist` for the math inverse number. If the
    /// puzzle is valid (inverse number (`cnt`) is the power of 2),
    /// we create double deque for Two-way `BFS`, one start by `src`
    /// and the other start by `dist`. We can simply setup a `hash
    /// map` to record whether a step is repeated and who passby it.
    /// For moving the blocks, the valid directions are filtered by
    /// the `bound` of the board. Due to the borrowed rules, I swap
    /// the raw ptr to avoid double mutable borrow (which actually
    /// safe sometime).
    pub fn sliding_puzzle<T: Clone + Eq + Hash>(
        src: &Vec<Vec<T>>,
        dist: &Vec<Vec<T>>,
        nul: T,
    ) -> Option<usize> {
        assert_eq!(src.len(), dist.len());
        assert_eq!(
            src.get(0).expect("Get bound failed!").len(),
            dist.get(0).expect("Get bound failed!").len()
        );
        #[rustfmt::skip]
        let bound = (
            (0, src.len() - 1),
            (0, src[0].len() - 1)
        );
        let direct = |pos: (usize, usize)| {
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
            direct
        };
        let (pos1, pos2, cnt) = {
            let mut vec = Vec::new();
            let mut map = HashMap::new();
            let mut pos1 = None;
            let mut pos2 = None;
            let mut idx = 0;
            for x in bound.0 .0..=bound.0 .1 {
                for y in bound.1 .0..=bound.1 .1 {
                    if src[x][y] == nul {
                        pos1 = Some((x, y));
                    } else {
                        vec.push(&src[x][y]);
                    }
                    if dist[x][y] == nul {
                        pos2 = Some((x, y));
                    } else {
                        map.insert(&dist[x][y], idx);
                        idx += 1;
                    }
                }
            }
            let mut vec = vec
                .into_iter()
                .map(|v| map.get(v).expect("Board is invalid!"))
                .collect::<Vec<&usize>>();
            let mut cnt = 0usize;
            for end in (1..vec.len()).rev() {
                let bak = cnt;
                for idx in 0..end {
                    if vec[idx] > vec[idx + 1] {
                        vec.swap(idx, idx + 1);
                        cnt += 1;
                    }
                }
                if bak == cnt {
                    break;
                }
            }
            (
                pos1.expect("Get position failed!"),
                pos2.expect("Get position failed!"),
                cnt,
            )
        };
        if cnt & 1 == 0 {
            let mut map = HashMap::new();
            let mut deque1 = VecDeque::new();
            let mut deque2 = VecDeque::new();
            deque1.push_back((src.clone(), pos1, 0));
            deque2.push_back((dist.clone(), pos2, 0));
            let search = |vec: &Vec<Vec<T>>,
                          deque: &mut VecDeque<(Vec<Vec<T>>, (usize, usize), usize)>,
                          pos: (usize, usize),
                          step: usize| {
                for next in direct(pos) {
                    let mut vec = vec.clone();
                    unsafe {
                        ptr::swap(
                            &mut vec[next.0][next.1] as *mut T,
                            &mut vec[pos.0][pos.1] as *mut T,
                        );
                    }
                    deque.push_back((vec, next, step + 1));
                }
            };
            while match deque1.pop_front() {
                Some((src, pos1, step1)) => {
                    match map.get(&src) {
                        Some((2, step2)) => return Some(step1 + step2),
                        None => {
                            map.insert(src.clone(), (1, step1));
                            search(&src, &mut deque1, pos1, step1);
                        }
                        _ => (),
                    }
                    true
                }
                None => false,
            } && match deque2.pop_front() {
                Some((dist, pos2, step2)) => {
                    match map.get(&dist) {
                        Some((1, step1)) => return Some(step1 + step2),
                        None => {
                            map.insert(dist.clone(), (2, step2));
                            search(&dist, &mut deque2, pos2, step2);
                        }
                        _ => (),
                    }
                    true
                }
                None => false,
            } {}
        }
        None
    }

    /// Question 4 - Eight Gueens Problem (0 ms 2.3 MB)
    ///
    /// Write an algorithm to print all ways of arranging n queens
    /// on an n x n chess board so that none of them share the same
    /// row, column, or diagonal. In this case, "diagonal" means all
    /// diagonals, not just the two that bisect the board.
    ///
    /// <https://leetcode-cn.com/problems/eight-queens-lcci>
    ///
    /// This problem can be simply solved by `DFS`. Firstly, we build
    /// a "map" with each line to it's column, then we fork the "map"
    /// each different choose (column). The only thing we should pay
    /// attention is cut the fork invalid (line 319 ~ 328). When the
    /// recursion on the top (map forked is full), we convert the map
    /// to result's element (line 307 ~ 316).
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
    /// then modify the line 322.
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
                            if map[i] == y || i.abs_diff(x) == map[i].abs_diff(y) {
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
