use crate::Solution;

#[test]
fn freq_alphabets_1() {
    assert_eq!(Solution::freq_alphabets("10#11#12"), "jkab");
}

#[test]
fn freq_alphabets_2() {
    assert_eq!(Solution::freq_alphabets("1326#"), "acz");
}

#[test]
fn freq_alphabets_3() {
    assert_eq!(Solution::freq_alphabets("25#"), "y");
}

#[test]
fn freq_alphabets_4() {
    assert_eq!(
        Solution::freq_alphabets("12345678910#11#12#13#14#15#16#17#18#19#20#21#22#23#24#25#26#"),
        "abcdefghijklmnopqrstuvwxyz"
    );
}

#[test]
#[rustfmt::skip]
fn plates_between_candles_1() {
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
fn plates_between_candles_2() {
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
