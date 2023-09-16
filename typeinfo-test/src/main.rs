mod expanded;

use typeinfo_macro::Reflect;

#[derive(Reflect)]
struct Foo<T> {
    field: T,
}

fn main() {
    println!("{:?}", Foo::<i32>::typeinfo());
}
