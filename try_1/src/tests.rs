use crate::Solution;

#[test]
#[rustfmt::skip]
fn josephs_ring() {
    let result = Solution::josephs_ring(10, 4);
    assert_eq!(
        result,
        vec![
            false, true,  true,  false, false,
            true,  false, false, true,  false,
            true,  false, true,  true,  false,
            false, true,  true,  true,  false,
        ]
    );
}

#[test]
fn is_palindrome_1() {
    assert_eq!(Solution::is_palindrome("madam, i am a man."), false);
}

#[test]
fn is_palindrome_2() {
    assert_eq!(Solution::is_palindrome("madam, I'm adam."), true);
}

#[test]
fn is_palindrome_3() {
    assert_eq!(Solution::is_palindrome("0P"), false); // leetcode
}

#[test]
fn tower_of_hanoi() {
    let result = Vec::from_iter((0..10).rev());
    println!("{:?}", result);
    let mut src = result.clone();
    let mut tmp = Vec::new();
    let mut dist = Vec::new();
    Solution::tower_of_hanoi(&mut src, &mut tmp, &mut dist);
    assert_eq!(src, vec![]);
    assert_eq!(tmp, vec![]);
    assert_eq!(dist, result);
}

#[test]
fn full_permutation() {
    assert_eq!(
        Solution::full_permutation(&vec![1, 2, 3]),
        vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 2, 1],
            vec![3, 1, 2],
        ]
    );
}
