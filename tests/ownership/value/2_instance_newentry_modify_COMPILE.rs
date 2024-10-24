#![allow(warnings)]

fn main() {
    let mut target: usize = 0;

    let target_entry = target;

    target = 42;

    println!("{}", target);
    println!("{}", target_entry);
}
