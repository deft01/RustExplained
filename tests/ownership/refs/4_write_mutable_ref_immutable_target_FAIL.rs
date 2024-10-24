#![allow(warnings)]

fn main() {
    let target: usize = 0;

    let ref_foo = &mut target;
    let ref_bar = &mut target;
    let ref_baz = &mut target;

    println!("{}", ref_baz);
}
