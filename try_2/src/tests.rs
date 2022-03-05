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
fn red_and_black() {
    let room = vec![
        vec!['.', '.', '.', '.', '#'],
        vec!['.', '.', '.', '.', '.'],
        vec!['#', '@', '.', '.', '.'],
        vec!['.', '#', '.', '.', '#'],
    ];
    for vec in &room {
        println!("{:?}", vec);
    }
    let mut pos = (0, 0);
    for (x, vec) in room.iter().enumerate() {
        for (y, &c) in vec.iter().enumerate() {
            if c == '@' {
                pos = (x, y);
            }
        }
    }
    assert_eq!(Solution::red_and_black(&room, pos), 15);
}

#[test]
fn sliding_puzzle() {
    let src = vec![vec![1, 2, 3], vec![0, 8, 4], vec![7, 6, 5]];
    let dist = vec![vec![1, 0, 3], vec![8, 2, 4], vec![7, 6, 5]];
    assert_eq!(Solution::sliding_puzzle(&src, &dist, 0).unwrap(), 2);
}

#[test]
fn solve_n_queens() {
    let result = Solution::solve_n_queens(8);
    assert_eq!(result.len(), 92);
    println!("{:#?}", result);
}
