use super::Solution;

#[test]
#[rustfmt::skip]
fn test1() {
    assert_eq!(
        Solution::circle_game(
            &vec![(3, 3, 1), (3, 2, 1)],
            &vec![(4, 3)],
            2
        ),
        1
    );
}

#[test]
fn test2() {
    assert_eq!(
        Solution::circle_game(
            &vec![(1, 3, 2), (4, 3, 1), (7, 1, 2)],
            &vec![(1, 0), (3, 3)],
            4
        ),
        2
    );
}
