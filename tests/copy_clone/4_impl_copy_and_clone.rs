#![allow(warnings)]

#[derive(Copy, Clone)]
struct Foo {
    x: i32,
}

fn main() {
    let foo1 = Foo { x: 0 };
    let foo2 = foo1;         // Implicit `Copy`
    let foo3 = foo1.clone(); // Explicit `Clone`

    assert_eq!(foo1.x, foo2.x);
    assert_eq!(foo2.x, foo3.x);
}

