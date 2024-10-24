#![allow(warnings)]

#[derive(Debug)]
struct Boo { u: usize }

fn main() {
    let mut target = Boo { u: 0 };

    let target_entry = target;

    target.u = 42;

    println!("{:#?}", target);
    println!("{:#?}", target_entry);
}
