use crate::Solution;
use rand::prelude::*;

#[test]
fn quick_sort_1() {
    let mut vec = (0..10000).into_iter().collect();
    Solution::quick_sort(&mut vec, |a, b| a <= b);
    assert_eq!(vec, (0..10000).into_iter().collect::<Vec<usize>>());
}

#[test]
fn quick_sort_2() {
    let mut vec = (0..10000).into_iter().collect();
    Solution::quick_sort(&mut vec, |a, b| a >= b);
    assert_eq!(vec, (0..10000).rev().into_iter().collect::<Vec<usize>>());
}

#[test]
fn quick_sort_3() {
    let mut vec = (0..10000)
        .into_iter()
        .map(|_| thread_rng().gen_range(0..10000))
        .collect::<Vec<usize>>();
    Solution::quick_sort(&mut vec, |a, b| a <= b);
}

#[test]
fn merge_sort_1() {
    let mut vec = (0..10000).into_iter().collect();
    Solution::merge_sort(&mut vec, |a, b| a <= b);
    assert_eq!(vec, (0..10000).into_iter().collect::<Vec<usize>>());
}

#[test]
fn merge_sort_2() {
    let mut vec = (0..10000).into_iter().collect();
    Solution::merge_sort(&mut vec, |a, b| a >= b);
    assert_eq!(vec, (0..10000).rev().into_iter().collect::<Vec<usize>>());
}

#[test]
fn merge_sort_3() {
    let mut vec = (0..10000)
        .into_iter()
        .map(|_| thread_rng().gen_range(0..10000))
        .collect::<Vec<usize>>();
    Solution::merge_sort(&mut vec, |a, b| a <= b);
}
