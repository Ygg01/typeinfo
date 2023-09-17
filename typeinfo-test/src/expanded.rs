// #![feature(prelude_import)]
// #[prelude_import]
// use std::prelude::rust_2021::*;
// #[macro_use]
// extern crate std;
// mod expanded {}
// use typeinfo_macro::Reflect;
// struct Foo<'a, T: 'a> {
//     field: &'a T,
// }
// #[allow(non_snake_case, non_camel_case_types)]
// impl<'a, T: 'a> Foo<'a, T> {
//     const fn typeinfo() -> ::typeinfo_core::Type {
//         use core::alloc::Layout;
//         ::typeinfo_core::Type {
//             name: &"Foo",
//             inner: ::typeinfo_core::InnerTypeInfo::None,
//             layout: Layout::new::<Foo<'a, T>>(),
//             generics: &[],
//             lifetimes: &[
//                 ::typeinfo_core::LifetimeTy {
//                     name: "a",
//                 },
//             ],
//         }
//     }
// }
// fn main() {
//     {
//         ::std::io::_print(format_args!("{0:?}\n", Foo::< i32 >::typeinfo()));
//     };
//     {
//         ::std::io::_print(format_args!("{0:?}\n", Foo::< i32 >::typeinfo().lifetimes));
//     };
// }
