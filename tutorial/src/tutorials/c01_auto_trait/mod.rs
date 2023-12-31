pub mod s1;
pub mod s2;
pub mod s3;
pub mod s4;
pub mod s5;
pub mod s6;

use core::fmt::Display;

use itertools::Itertools;

pub trait CanPrintItems {
    fn print_items(self);
}

#[test]
fn test() {
    vec![1, 2, 3].print_items();
}

pub trait CanFormatItems {
    fn format_items(&mut self) -> String;
}

impl<I> CanFormatItems for I
where
    I: Itertools + Clone,
    I::Item: Display + Eq + Ord,
{
    fn format_items(&mut self) -> String {
        self.join(", ")
    }
}

impl<T> CanPrintItems for T
where
    T: IntoIterator,
    T::IntoIter: CanFormatItems,
{
    fn print_items(self) {
        println!("{}", self.into_iter().format_items());
    }
}
