use core::fmt::{Debug, Display};

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
    Foo(vec![1, 2, 3].into_iter()).print_items();
}

// Consumer trait
pub trait CanFormatItems {
    fn format_items(self) -> String;
}

// Provider trait
pub trait ItemsFormatter<Context> {
    fn format_items(context: Context) -> String;
}

impl<Context> CanFormatItems for Context
where
    Context: HasComponents,
    Context::Components: ItemsFormatter<Context>,
{
    fn format_items(self) -> String {
        Context::Components::format_items(self)
    }
}

pub trait HasComponents {
    type Components;
}

impl<I> HasComponents for Foo<I> {
    type Components = FormatItemsWithDisplay;
}

impl<I> HasComponents for Bar<I> {
    type Components = FormatItemsWithDebug;
}

impl<I> HasComponents for Baz<I> {
    type Components = FormatItemsWithDisplay;
}

struct Foo<T>(pub T);

struct Bar<T>(pub T);

struct Baz<T>(pub T, pub u32);

pub struct FormatItemsWithDebug;

impl<Context> ItemsFormatter<Context> for FormatItemsWithDebug
where
    Context: HasInner,
    Context::Inner: Itertools,
    <Context::Inner as Iterator>::Item: Debug,
{
    fn format_items(context: Context) -> String {
        context.into_inner().map(|x| format!("{:?}", x)).join(",")
    }
}

pub struct FormatItemsWithDisplay;

pub trait HasInner {
    type Inner;

    fn into_inner(self) -> Self::Inner;
}

impl<I> HasInner for Foo<I> {
    type Inner = I;

    fn into_inner(self) -> Self::Inner {
        self.0
    }
}

impl<I> HasInner for Bar<I> {
    type Inner = I;

    fn into_inner(self) -> Self::Inner {
        self.0
    }
}

impl<I> HasInner for Baz<I> {
    type Inner = I;

    fn into_inner(self) -> Self::Inner {
        self.0
    }
}

impl<Context> ItemsFormatter<Context> for FormatItemsWithDisplay
where
    Context: HasInner,
    Context::Inner: Itertools + Clone,
    <Context::Inner as Iterator>::Item: Display + Eq + Ord + HasSeparator,
{
    fn format_items(context: Context) -> String {
        context
            .into_inner()
            .join(<Context::Inner as Iterator>::Item::SEPARATOR)
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
