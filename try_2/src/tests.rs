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
fn sliding_puzzle() {}

#[test]
fn solve_n_queens() {}
