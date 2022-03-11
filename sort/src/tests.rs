use crate::Solution;
use rand::prelude::*;

#[test]
fn quick_sort_1() {
    let mut src = (0..10_000).into_iter().collect::<Vec<usize>>();
    let dist = (0..10_000).into_iter().collect::<Vec<usize>>();
    Solution::quick_sort(&mut src, |a, b| a < b);
    assert_eq!(src, dist);
}

#[test]
fn quick_sort_2() {
    let mut src = (0..10_000).into_iter().collect::<Vec<usize>>();
    let dist = (0..10_000).rev().into_iter().collect::<Vec<usize>>();
    Solution::quick_sort(&mut src, |a, b| a > b);
    assert_eq!(src, dist);
}

#[test]
fn quick_sort_3() {
    let mut src = (0..1_000_000)
        .into_iter()
        .map(|_| thread_rng().gen_range(0..1_000_000))
        .collect::<Vec<usize>>();
    let mut dist = src.clone();
    Solution::quick_sort(&mut src, |a, b| a < b);
    dist.sort();
    assert_eq!(src, dist);
}

#[test]
fn merge_sort_1() {
    let mut src = (0..1_000_000).into_iter().collect::<Vec<usize>>();
    let dist = (0..1_000_000).into_iter().collect::<Vec<usize>>();
    Solution::merge_sort(&mut src, |a, b| a < b);
    assert_eq!(src, dist);
}

#[test]
fn merge_sort_2() {
    let mut src = (0..1_000_000).into_iter().collect::<Vec<usize>>();
    let dist = (0..1_000_000).rev().into_iter().collect::<Vec<usize>>();
    Solution::merge_sort(&mut src, |a, b| a > b);
    assert_eq!(src, dist);
}

#[test]
fn merge_sort_3() {
    let mut src = (0..1_000_000)
        .into_iter()
        .map(|_| thread_rng().gen_range(0..1_000_000))
        .collect::<Vec<usize>>();
    let mut dist = src.clone();
    Solution::merge_sort(&mut src, |a, b| a < b);
    dist.sort();
    assert_eq!(src, dist);
}
