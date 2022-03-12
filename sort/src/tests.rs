use crate::Solution;
use rand::prelude::*;

#[test]
fn quick_sort_1() {
    assert!(Solution::quick_sort(
        &(-10_000..10_000).into_iter().collect::<Vec<i16>>(),
        |a, b| a < b
    )
    .is_sorted());
}

#[test]
fn quick_sort_2() {
    assert!(Solution::quick_sort(
        &(-10_000..10_000).into_iter().collect::<Vec<i16>>(),
        |a, b| a > b
    )
    .is_sorted_by(|a, b| b.partial_cmp(a)));
}

#[test]
fn quick_sort_3() {
    assert!(Solution::quick_sort(
        &(0..2_000_000)
            .into_iter()
            .map(|_| thread_rng().gen_range(-1_000_000..1_000_000))
            .collect::<Vec<i32>>(),
        |a, b| a < b
    )
    .is_sorted());
}

#[test]
fn merge_sort_1() {
    assert!(Solution::merge_sort(
        &(-1_000_000..1_000_000).into_iter().collect::<Vec<i32>>(),
        |a, b| a < b,
    )
    .is_sorted());
}

#[test]
fn merge_sort_2() {
    assert!(Solution::merge_sort(
        &(-1_000_000..1_000_000).into_iter().collect::<Vec<i32>>(),
        |a, b| a > b,
    )
    .is_sorted_by(|a, b| b.partial_cmp(a)));
}

#[test]
fn merge_sort_3() {
    assert!(Solution::merge_sort(
        &(0..2_000_000)
            .into_iter()
            .map(|_| thread_rng().gen_range(-1_000_000..1_000_000))
            .collect::<Vec<i32>>(),
        |a, b| a < b
    )
    .is_sorted());
}
