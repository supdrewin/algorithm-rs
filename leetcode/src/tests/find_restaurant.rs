use super::Solution;

#[test]
fn test1() {
    assert_eq!(
        Solution::find_restaurant(
            vec![
                "Shogun".to_string(),
                "Tapioca Express".to_string(),
                "Burger King".to_string(),
                "KFC".to_string()
            ],
            vec![
                "Piatti".to_string(),
                "The Grill at Torrey Pines".to_string(),
                "Hungry Hunter Steakhouse".to_string(),
                "Shogun".to_string()
            ]
        ),
        vec!["Shogun"]
    )
}

#[test]
fn test2() {
    assert_eq!(
        Solution::find_restaurant(
            vec![
                "Shogun".to_string(),
                "Tapioca Express".to_string(),
                "Burger King".to_string(),
                "KFC".to_string(),
            ],
            vec![
                "KFC".to_string(),
                "Shogun".to_string(),
                "Burger King".to_string()
            ]
        ),
        vec!["Shogun"]
    )
}
