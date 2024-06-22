# What is Rc?

## Limitation of Box

The limitation of the `Box` type is that it allows only one reference to a value
allocated in heap. So this prohibits the user from defining structures where a single
value in the heap can have multiple references like Graph data structure.

`Rc` type allows a value in the heap to have multiple references.

## Is it possible to mutate a value of type Rc?

Rust borrowship rule states that there could be an exclusive mutable referene or
multiple read only references. Since a value enclosed by `Rc` type can have multiple
references it cannot be mutated.

## How it works under the hood?

`Rc` is a smart pointer which can be demonstrated using the following code sample which won't compile :),

```rust
struct Rc<T> {
    ref_value: &'static T,
    ref_counter: &'static usize,
    // Here the static lifetime parameter signifies that the value and counter are stored in heap.
}

// When a Rc is cloned, we increment the counter to keep track of number of active references.
// Then we copy the reference to the value and the counter.
// NOTE: The actual value in the heap is not copied.
impl Rc<T> {
    fn clone(&mut self) -> Self {
        unsafe {
            *self.ref_counter += 1; 
        }
        Rc {
            ref_value: self.ref_value,
            ref_counter: self.ref_counter,
        }
    }
}

// When a Rc value goes out of scope the drop method of the Drop trait is invoked.
// It works like the below demonstration

impl Drop<T> for Rc<T> {
    fn drop(&mut self) {
        unsafe {
            *self.ref_counter -= 1;
            if *self.ref_counter <= 0 {
                free(self.ref_counter);
                free(self.ref_value);
                // Refering to os free sys call
                // Look into linux man page https://linux.die.net/man/3/free
            }
        }
    }
}
```

`Rc` type can only be used in single threaded case as it does not protect against race
condtions when mutating the `ref_counter`. For thread safe alternative refer `Arc`.

## Question

Given this fact one might wonder why the compiler might not able to determine during compile
time itself when the reference count will become zero?

Why this question arises is because of the fact that the examples given in the rust book and docs
are very simplistic that in those cases it is indeed possible for the compiler to detect when the
value goes out of scope i.e the reference count will become zero.

## Example where the compiler cannot determine when the reference count will become zero

```rust
fn f(r: Rc<T>) {
    // Does something and goes out of scope
}

fn main() {
    let n: u32 = get_input_from_user();

    let some_value = Rc::new(Some(45));
    for i in 0..n {
        f(some_value.clone());
    }
}
```
In the above example it is not possible during the compile time to determine when to drop
the value `Some(45)` from heap, because the function `f` is invoked `n` number of times, where
during each time the value is cloned and `n` can only be known at run time.
