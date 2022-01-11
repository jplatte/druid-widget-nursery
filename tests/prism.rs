#![cfg(feature = "derive")]

use druid_widget_nursery::prism::Prism;

#[derive(Prism)]
enum MyOption<T> {
    Some(T),
    None,
}

#[derive(Prism)]
enum CLike {
    A,
    B,
    C,
}

#[derive(Prism)]
enum Complex {
    First,
    Second(),
    Third(u32),
    Fourth(String, Box<Complex>),
}
