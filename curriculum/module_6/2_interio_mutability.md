# When Unique mutable borrow rule is too restrictive

In order to ensure memory safety and to have consistent readability for single threaded and
multi threaded code, rust has the following rules

1. A single mutable reference `&mut T` (or)
1. Multiple immutable references `&T`

These rules are too restrictive to write performant single threaded and multi threaded code.
In this chapter we will see scenarios in single threaded code where these rules makes it impossible
to write efficient code.
Since Rust promises performance it provides us with types that can help the programmer to handle
these scenarios without comprimising on speed!

## Scenario 1: Caching of computations

Consider,

```rust
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance_from_origin(&self) -> f64 {
        println!("doing the complex mathematical powers and square root");
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn demo() {
    let p = Point {x: 23.3, y: -45.1342};
    println!("{}", p.distance_from_origin());
    println!("{}", p.distance_from_origin());
}
```

If you run the example, you can see that everytime the expensive mathematical computations are
carried out.

To do the computation once but still have an immutable variable/reference we make use of the
[Cell](https://doc.rust-lang.org/std/cell/struct.Cell.html) type.


```rust
use std::cell::Cell;

struct Point {
    x: f64,
    y: f64,
    cache: Cell<Option<f64>>, // because initially the cache will be empty/none.
}
// By this stage you should know how to initialize a value for the above struct

impl Point {
    fn distance_from_origin(&self) -> f64 {
        match cache.get() {
            Some(d) => {
                println!("returning  cached value");
                d
            },
            None => {
                println!("doing the complex mathematical powers and square root");
                let d = (self.x.powi(2) + self.y.powi(2)).sqrt();
                self.set(Some(d));
                d
            }
        }
    }
}

fn demo() {
    let p = Point {x: 23.3, y: -45.1342};
    println!("{}", p.distance_from_origin());
    println!("{}", p.distance_from_origin());
}
```

Notice that in both the implementations we take an immutable reference but in the latter mutation
happens interiorly.

## Scenario 2: Tracking the clicks

```rust
use std::cell::Cell;

struct Button {
    label: String,
    click_count: Cell<u32>,
}

impl Button {
    fn new(label: &str) -> Self {
        Button {
            label: label.to_string(),
            click_count: Cell::new(0),
        }
    }

    fn click(&self) {
        let count = self.click_count.get();
        self.click_count.set(count + 1);
    }

    fn get_click_count(&self) -> u32 {
        self.click_count.get()
    }
}
```

NOTE: In both the scenarios that we saw, `Cell` comes in handy when we need to have a certain Copyable 
field mutable. So there is no overhead in using `Cell` and it is one of zero cost optimization.

## Borrow checking at runtime

When you want to have interior mutability of types which are not copyable use [RefCell](https://doc.rust-lang.org/std/cell/struct.RefCell.html).
This comes with runtime over head because it checks in runtime whether there is a unique mutable borrow
and panics in cases where borrow checking rules are violated.

## Scenario 3: Tree with mutable nodes

Tree is a special case of a [connected graph](https://en.wikipedia.org/wiki/Connectivity_(graph_theory)) which does not contain any cycle.
A node in a tree can have muliple references to it, so we need to use `Rc` in conjunction with `RefCell`.

The example rust code to represent a mutable TreeNode.

```rust
use std::cell::RefCell;
use std::rc::Rc;

struct TreeNode {
    value: RefCell<i32>,
    children: Vec<Rc<TreeNode>>,
}

impl TreeNode {
    fn new(value: i32) -> Rc<TreeNode> {
        Rc::new(TreeNode {
            value: RefCell::new(value),
            children: Vec::new(),
        })
    }

    fn add_child(parent: &Rc<TreeNode>, child: Rc<TreeNode>) {
        parent.children.push(child);
    }

    fn set_value(&self, new_value: i32) {
        *self.value.borrow_mut() = new_value;
    }

    fn get_value(&self) -> i32 {
        *self.value.borrow()
    }
}
```

One should observe that without `Cell` and `RefCell` we would not be able to write safe Rust code.
Yes you guessed it! Both these types uses unsafe type [UnsafeCell](https://doc.rust-lang.org/stable/std/cell/struct.UnsafeCell.html) under the hood.
