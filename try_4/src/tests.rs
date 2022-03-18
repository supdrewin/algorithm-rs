use super::Solution;

#[test]
fn recover_lost_records() {
    assert_eq!(
        Solution::recover_lost_records(&vec![(2, 1), (3, 3), (4, 2)]),
        vec![(2, 1), (3, 2), (4, 2)]
    );
}

#[test]
fn count_enemies() {
    assert_eq!(
        Solution::count_enemies(
            &mut vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            &vec![
                ("Query".to_string(), 1, 3),
                ("Add".to_string(), 3, 6),
                ("Query".to_string(), 2, 7),
                ("Sub".to_string(), 10, 2),
                ("Add".to_string(), 6, 3),
                ("Query".to_string(), 3, 10)
            ]
        ),
        vec![6, 33, 59]
    );
}

#[test]
fn cows_eat_clover() {
    assert_eq!(
        Solution::cows_eat_clover(&vec![(1, 2), (0, 3), (3, 4)]),
        vec![1, 0, 0]
    );
}
