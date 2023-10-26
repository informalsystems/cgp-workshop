use core::fmt::{Debug, Display};
use core::marker::PhantomData;

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
    Baz(vec![1, 2, 3].into_iter(), 3).print_items();
}

// Consumer trait
pub trait CanFormatItems {
    fn format_items(&mut self) -> String;
}

// Provider trait
pub trait ItemsFormatter<Context> {
    fn format_items(context: &mut Context) -> String;
}

impl<I> CanFormatItems for Foo<I>
where
    FormatItemsWithDisplay: ItemsFormatter<I>,
{
    fn format_items(&mut self) -> String {
        FormatItemsWithDisplay::format_items(&mut self.0)
    }
}

impl<I> CanFormatItems for Bar<I>
where
    FormatItemsWithDebug: ItemsFormatter<I>,
{
    fn format_items(&mut self) -> String {
        FormatItemsWithDebug::format_items(&mut self.0)
    }
}

impl<I> CanFormatItems for Baz<I>
where
    FormatItemsWithDisplay: ItemsFormatter<I>,
{
    fn format_items(&mut self) -> String {
        FormatItemsWithDisplay::format_items(&mut self.0)
    }
}

struct Foo<T>(pub T);

struct Bar<T>(pub T);

struct Baz<T>(pub T, pub u32);

pub struct FormatItemsWithDebug;

impl<I> ItemsFormatter<I> for FormatItemsWithDebug
where
    I: Itertools,
    I::Item: Debug,
{
    fn format_items(iter: &mut I) -> String {
        iter.map(|x| format!("{:?}", x)).join(",")
    }
}

pub struct FormatItemsWithDisplay;

impl<I> ItemsFormatter<I> for FormatItemsWithDisplay
where
    I: Itertools + Clone,
    I::Item: Display + Eq + Ord + HasSeparator,
{
    fn format_items(iter: &mut I) -> String {
        iter.join(I::Item::SEPARATOR)
    }
}

impl<T> CanPrintItems for T
where
    T: CanFormatItems,
{
    fn print_items(mut self) {
        println!("{}", self.format_items());
    }
}
