#![allow(warnings)]

fn main() {
    let mut target: usize = 0;

    let ref_foo = &target;
    let ref_bar = &target;
    let ref_baz = &target;

    println!("{}", ref_bar);
}
