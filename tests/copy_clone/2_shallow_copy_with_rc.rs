#![allow(warnings)]
use std::rc::Rc;

#[derive(Clone)]
struct Bar {
    name: Rc<String>,
}

fn main() {
    let foo = Bar {
        name: Rc::new(String::from("Pasolini")),
    };
    
    let cloned_foo = foo.clone();

    assert_eq!(Rc::as_ptr(&foo.name), Rc::as_ptr(&cloned_foo.name));
}

