use core::fmt::Display;

use itertools::Itertools;

pub trait CanPrintItems {
    fn print_items(self);
}

pub trait HasSeparator {
    const SEPARATOR: &'static str;
}

impl HasSeparator for u64 {
    const SEPARATOR: &'static str = ",";
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
    I::Item: Display + Eq + Ord + HasSeparator,
{
    fn format_items(&mut self) -> String {
        self.join(I::Item::SEPARATOR)
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
