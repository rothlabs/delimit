macro_rules! make_printer {
    ($($element: ident: $ty: ty),*) => {
        struct Foo { $($element: $ty),* }
    }
}

make_printer!(x: i32, y: String);