use cgp_core::prelude::*;
use core::fmt::{Debug, Display};

use itertools::Itertools;

pub trait CanPrintItems {
    fn print_items(self);
}

#[derive_component(SeparatorGetterComponent, SeparatorGetter<Context>)]
pub trait HasSeparator {
    fn separator(&self) -> String;
}

pub struct CommaSeparator;

impl<Context> SeparatorGetter<Context> for CommaSeparator {
    fn separator(_context: &Context) -> String {
        ",".to_string()
    }
}

#[test]
fn test() {
    Foo(vec![1, 2, 3].into_iter()).print_items();
    Bar(vec![1, 2, 3].into_iter()).print_items();
    Baz(vec![1, 2, 3].into_iter(), "|".to_string()).print_items();
}

// Consumer trait
#[derive_component(ItemsFormatterComponent, ItemsFormatter<Context>)]
pub trait CanFormatItems {
    fn format_items(self) -> String;
}

impl<I> HasComponents for Foo<I>
where
    I: Async,
{
    type Components = FooComponents;
}

impl<I> HasComponents for Bar<I>
where
    I: Async,
{
    type Components = BarComponents;
}

impl<I> HasComponents for Baz<I>
where
    I: Async,
{
    type Components = BazComponents;
}

struct Foo<T>(pub T);

struct Bar<T>(pub T);

struct Baz<T>(pub T, pub String);

pub struct FooComponents;
pub struct BarComponents;
pub struct BazComponents;

delegate_components!(
    FooComponents;
    ItemsFormatterComponent: FormatItemsWithDebug,
);

delegate_components!(
    BarComponents;
    ItemsFormatterComponent: FormatItemsWithDisplay,
    SeparatorGetterComponent: CommaSeparator,
);

delegate_components!(
    BazComponents;
    ItemsFormatterComponent: FormatItemsWithDisplay,
);

delegate_components!(
    BazComponents;
    SeparatorGetterComponent: GetBazSepatator,
);

pub struct GetBazSepatator;

impl<I> SeparatorGetter<Baz<I>> for GetBazSepatator {
    fn separator(context: &Baz<I>) -> String {
        context.1.clone()
    }
}

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
    Context: HasInner + HasSeparator,
    Context::Inner: Itertools + Clone,
    <Context::Inner as Iterator>::Item: Display + Eq + Ord,
{
    fn format_items(context: Context) -> String {
        let separator = context.separator();
        context.into_inner().join(&separator)
    }
}

impl<T> CanPrintItems for T
where
    T: CanFormatItems,
{
    fn print_items(self) {
        println!("{}", self.format_items());
    }
}
