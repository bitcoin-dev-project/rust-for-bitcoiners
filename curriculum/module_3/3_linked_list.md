# Linked List in Rust

Linked List is a data structure which is optimal to store an arbitrary list
of ordered items, where the items are added and removed from the list frequently.

The linked list that we are going to implement is called *Singly Linked List*,
when given a node we can go to the next node, but we cannot go back to the previous node.

## How to define Linked List?

To answer that we need to answer the following questions,

* How to represent the items?
* How to link the items together?
* How to define the starting point of the linked list?

### How to represent the items?

Rust supports generic programming, since the items are arbitrary in nature let's
denote it using the generic variable `T`.

### How to link the items together?

The generic type `T` that we have chosen above does not have any information about
the linked list that we are trying to build. So let's define a new type,

```rust
struct Node<T> {
    val: T,
    next: Node<T>,
}
```

In the above definition the `next` field will point to the next node.

**Are we good to go?**
NO! The rust compiler is not happy about the fact that it cannot deduce the size of the
`Node` type at compile time. You may wonder why that is the case, as a definition similar
to this will succeed in programming languages like Java, C#, Scala, Swift etc.,
Because those languages by default will store every value of type `Node` in the heap and
the `Node::next` field will only hold the address of the value stored in the heap.

Contrary to the typical garbage collected languages, rust tries to put every value in the
stack first. It tries to compute how much size is required to store the entire linked list
in the stack and that could be infinite.

So as explained in the [previous chapter](./2_box.md), `Box` is used to solve the problem
as follows,

```rust
struct Node<T> {
    val: T,
    next: Box<Node<T>>,
}
```

**Are we done yet?**
No! How to represent the case that there is no next node? If you remember correctly
`Box` cannot represent null value, so we need `Option` to represent the absence of the next
node.

```rust
struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}
```

### How to define the starting point of the linked list?

This one is pretty simple,

```rust
struct LinkedList<T> {
    head: Node<T>,
}
```

With the above definition the first item, will be stored in the stack and rest of
them will be stored in the heap.
Pay close attention to the type of `LinkedList::head`.
