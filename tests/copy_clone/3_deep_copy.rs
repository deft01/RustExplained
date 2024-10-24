#![allow(warnings)]

#[derive(Debug, Clone)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

fn main() {
    let node1 = Node {
        value: 1,
        next: Some(Box::new(Node {
            value: 2,
            next: None,
        })),
    };

    let node2 = node1.clone(); // Deep copy 

    println!("Node1: {:#?}, Node2: {:#?}", node1, node2);

    assert_eq!(node1.value, node2.value);
    assert_eq!(node1.next.unwrap().value, node2.next.unwrap().value);
}

