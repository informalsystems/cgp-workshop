use itertools::Itertools;

pub fn join_nums(nums: &[i64]) -> String {
    nums.iter().format(", ").to_string()
}

#[test]
fn test_join_nums() {
    assert_eq!(join_nums(&vec![1, 2, 3]), "1, 2, 3");
}
