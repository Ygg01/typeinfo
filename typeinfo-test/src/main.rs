mod expanded;

use typeinfo_macro::Reflect;

#[derive(Reflect)]
struct Foo<'a, T: 'a> {
    field: &'a T,
}

fn main() {
    println!("{:?}", Foo::<i32>::typeinfo());
    println!("{:?}", Foo::<i32>::typeinfo().lifetimes);
}
