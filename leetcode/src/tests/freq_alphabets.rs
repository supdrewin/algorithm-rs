use super::Solution;

#[test]
fn test1() {
    assert_eq!(Solution::freq_alphabets("10#11#12"), "jkab");
}

#[test]
fn test2() {
    assert_eq!(Solution::freq_alphabets("1326#"), "acz");
}

#[test]
fn test3() {
    assert_eq!(Solution::freq_alphabets("25#"), "y");
}

#[test]
fn test4() {
    assert_eq!(
        Solution::freq_alphabets("12345678910#11#12#13#14#15#16#17#18#19#20#21#22#23#24#25#26#"),
        "abcdefghijklmnopqrstuvwxyz"
    );
}
