use core::fmt::Display;
use itertools::Itertools;

pub fn join_items(nums: impl Itertools<Item = impl Display>) -> String {
    nums.format(", ").to_string()
}

#[test]
fn test_join_items() {
    assert_eq!(join_items(vec![1, 2, 3].iter()), "1, 2, 3");
}
