use typeinfo_macro::Reflect;

#[derive(Reflect)]
struct Foo<'a, T: 'a> {
    field: &'a T,
}

fn main() {
    let x = Foo {
        field: &32i32,
    };
    println!("{:?}", x.typeinfo());
}
