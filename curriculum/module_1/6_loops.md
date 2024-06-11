# Loops in Rust

Loops in Rust are very intuitive if you have prior programming experience.

## Infinite loop with breaks

The loop keyword creates an infinite loop. You can exit the loop using the break statement.

```rust
fn main() {
    let mut counter = 0;

    loop {
        counter += 1;
        println!("Counter: {}", counter); // Output: 5

        if counter == 5 {
            break;
        }
    }
}
```

## Conditional loops

### while Loop

The while loop continues running while a condition is true.

```rust
fn main() {
    let mut counter = 0;

    while counter < 5 {
        counter += 1;
        println!("Counter: {}", counter); // Output: 5
    }
}
```

If you are experienced with c, java or js you might've used the following variation
of for loops extensively.
`for(i = 0; i < 5; i++)`,
In Rust we have to write the above loop using `while` loops as shown in the example.

## for loop

`for` loop in Rust works similar to `for` loop in python.
That is it works only with iterators. What are iterators?

### Iterator

An iterator is type which implements the following trait,

```rust
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

For example consider `Vec<T>`, then the `Item` will be `T`.
The `next` method will return `None` if we are at the end of the `Vec` or else `Some(T)`.

**Examples**

```rust
fn main() {
    // Iterating over a range
    for i in 0..5 {
        println!("i: {}", i);
    }

    // Iterating over an array
    let numbers = [1, 2, 3, 4, 5];

    for number in numbers.iter() {
        println!("Number: {}", number);
    }
}
```

It is to be noted that both the range and the array implements the `Iterator` trait,
If you try to for loop over a type which does not implement `Iterator` trait you will
get a compile time error.
