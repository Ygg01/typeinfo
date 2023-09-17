//
// #![feature(prelude_import)]
// #[prelude_import]
// use std::prelude::rust_2021::*;
// #[macro_use]
// extern crate std;
// use typeinfo_macro::Reflect;
// struct Foo<T> {
//     field: T,
// }
// #[allow(non_snake_case, non_camel_case_types)]
// impl<T> Foo<T> {
//     const fn typeinfo() -> ::typeinfo_core::Type {
//         use core::alloc::Layout;
//         ::typeinfo_core::Type {
//             name: &"Foo",
//             inner: ::typeinfo_core::TypeInner::None,
//             layout: Layout::new::<Foo>(),
//             generics: &[],
//             lifetimes: &[],
//         }
//     }
// }
// fn main() {
//     {
//         ::std::io::_print(format_args!("{0:?}\n", Foo::< i32 >::typeinfo()));
//     };
// }
