#![allow(unused)]
use std::rc::Rc;
struct LinkedListNode<T> {
    val: T,
    next: Option<Box<LinkedListNode<T>>>,
}
struct GraphNode<T> {
    data: T,
    edges: Vec<Rc<GraphNode<T>>>,
}

fn main() {}