use typeinfo_macro::Reflect;

#[derive(Reflect)]
struct X {
    field: i32,
}

fn main() {
    println!("{:?}", X::typeinfo());
}
