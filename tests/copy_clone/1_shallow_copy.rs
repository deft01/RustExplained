#![allow(warnings)]
use std::rc::Rc;

#[derive(Clone)]
struct Bar<'a> {
    name: &'a String,
}

fn main() {
    let foo = Bar {
        name: &String::from("Pasolini"),
    };

    let cloned_foo = foo.clone();

    assert_eq!(&foo.name, &cloned_foo.name);
}

