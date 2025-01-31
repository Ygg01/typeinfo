use typeinfo_macro::Reflect;

#[derive(Reflect)]
struct Foo<'a, T: 'a> {
    field: &'a T,
}

// impl<'a, T: 'a> Foo<'a, T> {
//     const fn typeinfo(&self) -> ::typeinfo_core::TypeInfo {
//         ::typeinfo_core::TypeInfo::Struct(::typeinfo_core::StructInfo {
//             name: "Foo",
//             lifetimes: &[typeinfo_core::LifetimeInfo { name: "T" }],
//             fields: &[],
//         })
//     }
// }



fn main() {
    let x = Foo {
        field: &32i32,
    };
    println!("{:?}", x.typeinfo());
}
