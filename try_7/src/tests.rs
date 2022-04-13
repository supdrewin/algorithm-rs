use super::Solution;

#[test]
fn catalan_number() {
    assert_eq!(Solution::catalan_number(0).to_string(), "1");
    assert_eq!(Solution::catalan_number(1).to_string(), "1");
    assert_eq!(Solution::catalan_number(2).to_string(), "2");
    assert_eq!(Solution::catalan_number(3).to_string(), "5");
    assert_eq!(Solution::catalan_number(4).to_string(), "14");
    assert_eq!(Solution::catalan_number(5).to_string(), "42");
    assert_eq!(Solution::catalan_number(6).to_string(), "132");
    assert_eq!(Solution::catalan_number(7).to_string(), "429");
    assert_eq!(Solution::catalan_number(8).to_string(), "1430");
    assert_eq!(Solution::catalan_number(9).to_string(), "4862");
    assert_eq!(Solution::catalan_number(10).to_string(), "16796");
    assert_eq!(Solution::catalan_number(11).to_string(), "58786");
    assert_eq!(Solution::catalan_number(12).to_string(), "208012");
    assert_eq!(Solution::catalan_number(13).to_string(), "742900");
    assert_eq!(Solution::catalan_number(14).to_string(), "2674440");
    assert_eq!(Solution::catalan_number(15).to_string(), "9694845");
    assert_eq!(Solution::catalan_number(16).to_string(), "35357670");
}
