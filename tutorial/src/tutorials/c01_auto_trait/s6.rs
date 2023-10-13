use core::fmt::Display;
use itertools::Itertools;

pub trait CanJoinItems {
    fn join_items(self) -> String;
}

impl<Context> CanJoinItems for Context
where
    Context: IntoIterator,
    Context::Item: Display,
{
    fn join_items(self) -> String {
        self.into_iter().format(", ").to_string()
    }
}

#[test]
fn test_join_items() {
    assert_eq!((&vec![1, 2, 3]).join_items(), "1, 2, 3");
}
