use super::Solution;

#[test]
#[rustfmt::skip]
fn test1() {
    assert_eq!(
        Solution::plates_between_candles(
            "**|**|***|",
            &vec![
                (2, 5),
                (5, 9),
            ],
        ),
        vec![2, 3],
    );
}

#[test]
#[rustfmt::skip]
fn test2() {
    assert_eq!(
        Solution::plates_between_candles(
            "***|**|*****|**||**|*",
            &vec![
                ( 1, 17),
                ( 4,  5),
                (14, 17),
                ( 5, 11),
                (15, 16),
            ],
        ),
        vec![9, 0, 0, 0, 0],
    );
}
