// Make me compile!
// #[derive(Debug, PartialEq, Eq)]
#[derive(Debug)]
struct Foo {
    capacity: i32,
}

fn main() {
    println!("{:?}", Foo { capacity: 3 });
}
