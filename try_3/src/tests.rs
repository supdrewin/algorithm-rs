use crate::Solution;

#[test]
fn find_all_suspects_1() {
    assert_eq!(
        Solution::find_all_suspects(
            &vec![
                vec![1, 2],
                vec![10, 13, 11, 12, 14],
                vec![0, 1],
                vec![99, 2],
            ],
            100
        ),
        4
    );
}

#[test]
#[rustfmt::skip]
fn find_all_suspects_2(){
    assert_eq!(
        Solution::find_all_suspects(
            &vec![
                vec![5],
                vec![1,2,3,4,5],
            ],
            200
        ),
        1
    );
}

#[test]
fn find_all_suspects_3() {
    assert_eq!(Solution::find_all_suspects(&vec![], 1), 1);
}

#[test]
fn get_postorder_traversal() {
    assert_eq!(
        Solution::get_postorder_traversal(
            &vec![1, 2, 4, 7, 3, 5, 8, 9, 6],
            &vec![4, 7, 2, 1, 8, 5, 9, 3, 6],
        ),
        vec![7, 4, 2, 8, 9, 5, 6, 3, 1]
    );
}

#[test]
fn recover_lost_records() {
    assert_eq!(
        Solution::recover_lost_records(&vec![(2, 1), (3, 3), (4, 2)]),
        vec![(2, 1), (3, 2), (4, 2)]
    );
}
