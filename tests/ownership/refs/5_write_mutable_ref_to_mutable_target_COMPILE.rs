#![allow(warnings)]

fn main() {
    let mut target: usize = 0;

    let ref_foo = &mut target;
    let ref_bar = &mut target;
    let ref_baz = &mut target;

    *ref_baz = 42;

    println!("{}", ref_baz);
}
