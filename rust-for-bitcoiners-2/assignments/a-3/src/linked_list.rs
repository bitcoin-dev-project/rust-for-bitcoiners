#![allow(unused)]

/* This module will be taught in the class */

struct LinkedList<T> {
    head: Node<T>,
}

struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}
