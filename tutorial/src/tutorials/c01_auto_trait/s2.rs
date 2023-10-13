use core::iter::Iterator;
use itertools::Itertools;

pub fn join_nums<'a>(nums: impl Iterator<Item = &'a i64>) -> String {
    nums.format(", ").to_string()
}

#[test]
fn test_join_nums() {
    assert_eq!(join_nums(vec![1, 2, 3].iter()), "1, 2, 3");
}
