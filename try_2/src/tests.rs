use crate::Solution;

#[test]
fn pentagram_game() {
    let result = Solution::pentagram_game();
    assert_eq!(result.len(), 12);
    for vec in &result {
        println!("{:?}", vec);
    }
}

#[test]
fn red_and_black_1() {
    let room = vec![
        vec!['.', '.', '.', '.', '#'],
        vec!['.', '.', '.', '.', '.'],
        vec!['#', '@', '.', '.', '.'],
        vec!['.', '#', '.', '.', '#'],
    ];
    for vec in &room {
        println!("{:?}", vec);
    }
    assert_eq!(Solution::red_and_black(&room, (2, 1)), 15);
}

#[test]
fn red_and_black_2() {
    let room = vec![
        vec!['.', '.', '.', '.', '#'],
        vec!['.', '.', '.', '.', '#'],
        vec!['.', '@', '#', '.', '.'],
        vec!['.', '.', '.', '.', '#'],
    ];
    for vec in &room {
        println!("{:?}", vec);
    }
    assert_eq!(Solution::red_and_black(&room, (2, 1)), 16);
}

#[test]
#[rustfmt::skip]
fn sliding_puzzle_1() {
    assert_eq!(
        Solution::sliding_puzzle(
            &vec![
                vec![1, 2, 3],
                vec![0, 8, 4],
                vec![7, 6, 5],
            ],
            &vec![
                vec![1, 0, 3],
                vec![8, 2, 4],
                vec![7, 6, 5],
            ],
            0,
        ),
        Some(2),
    );
}

#[test]
#[rustfmt::skip]
fn sliding_puzzle_2() {
    assert_eq!(
        Solution::sliding_puzzle(
            &vec![
                vec![3, 8, 5],
                vec![2, 0, 6],
                vec![1, 4, 7],
            ],
            &vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
                vec![7, 8, 0],
            ],
            0,
        ),
        Some(16),
    );
}

#[test]
#[rustfmt::skip]
fn sliding_puzzle_3() {
    assert_eq!(
        Solution::sliding_puzzle(
            &vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
                vec![8, 7, 0],
            ],
            &vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
                vec![7, 8, 0],
            ],
            0,
        ),
        None,
    );
}

#[test]
#[rustfmt::skip]
fn sliding_puzzle_4() {
    assert_eq!(
        Solution::sliding_puzzle(
            &vec![
                vec![1, 2, 3],
                vec![7, 8, 6],
                vec![5, 4, 0],
            ],
            &vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
                vec![7, 8, 0],
            ],
            0,
        ),
        None,
    );
}

#[test]
#[rustfmt::skip]
fn sliding_puzzle_5() {
    assert_eq!(
        Solution::sliding_puzzle(
            &vec![
                vec!['#', '.', ' '],
                vec!['.', '.', '.'],
                vec!['.', '.', '.'],
            ],
            &vec![
                vec!['.', '#', '.'],
                vec!['.', ' ', '.'],
                vec!['.', '.', '.'],
            ],
            ' ',
        ),
        Some(4),
    );
}

#[test]
#[rustfmt::skip]
fn sliding_puzzle_6() {
    assert_eq!(
        Solution::sliding_puzzle(
            &vec![
                vec![ 1, 14,  3,  4],
                vec![ 7,  0,  5,  8],
                vec![ 9, 11, 10, 12],
                vec![13,  2, 15,  6],
            ],
            &vec![
                vec![ 1,  2,  3,  4],
                vec![ 5,  6,  7,  8],
                vec![ 9, 10, 11, 12],
                vec![13, 14, 15,  0],
            ],
            0,
        ),
        Some(36),
    );
}

#[test]
#[ignore = "It's bound to timeout."]
#[rustfmt::skip]
fn sliding_puzzle_7() {
    assert_eq!(
        Solution::sliding_puzzle(
            &vec![
                vec![12, 14, 11,  4],
                vec![ 7,  0,  9,  8],
                vec![15, 10,  3, 13],
                vec![ 1,  2,  5,  6],
            ],
            &vec![
                vec![ 1,  2,  3,  4],
                vec![ 5,  6,  7,  8],
                vec![ 9, 10, 11, 12],
                vec![13, 14, 15,  0],
            ],
            0,
        ),
        Some(0),
    );
}

#[test]
fn solve_n_queens() {
    let result = Solution::solve_n_queens(8);
    assert_eq!(result.len(), 92);
    println!("{:#?}", result);
}
