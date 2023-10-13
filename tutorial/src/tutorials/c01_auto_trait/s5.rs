use core::fmt::Display;
use itertools::Itertools;

pub trait CanJoinItems {
    fn join_items(self) -> String;
}

impl<I> CanJoinItems for I
where
    I: Itertools,
    I::Item: Display,
{
    fn join_items(self) -> String {
        self.format(", ").to_string()
    }
}

#[test]
fn test_join_items() {
    assert_eq!(vec![1, 2, 3].iter().join_items(), "1, 2, 3");
}
