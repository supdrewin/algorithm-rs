use super::Solution;

#[test]
fn recover_lost_records() {
    assert_eq!(
        Solution::recover_lost_records(&vec![(2, 1), (3, 3), (4, 2)]),
        vec![(2, 1), (3, 2), (4, 2)]
    );
}
