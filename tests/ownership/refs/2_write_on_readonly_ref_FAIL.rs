#![allow(warnings)]

fn main() {
    let target: usize = 0;

    let ref_foo = &target;
    let ref_bar = &target;
    let ref_baz = &target;

    *ref_baz = 42;

    println!("{}", ref_baz);
}
