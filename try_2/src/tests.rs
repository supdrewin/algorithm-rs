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
fn solve_n_queens() {
    let result = Solution::solve_n_queens(8);
    assert_eq!(result.len(), 92);
    println!("{:#?}", result);
}
