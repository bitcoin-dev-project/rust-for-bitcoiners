# Option and Result

## Option Definition

```rust
enum Option<T> {
    None, // absence of Value
    Some(T), // presemce of value
}
```

`Option` is a container type in rust which is used to represent a value which may or may not be present.
The compiler will treat an Option value as schrodinger's cat, meaning it does not assume it's presence or
absence. So as a programmer we need to instruct the compiler on what to do in each of the case.

## How to use an Option

### Pattern matching

```rust
let v: Option<i32> = Some(5i32);
let z: i32 = v + 10; // FAILS to compile
```

#### Instruct what to do on either case

```rust
match v {
    Some(num) => num + 10,
    None => ???,
}
```

In case of missing value you can choose to terminate the program using `panic!` macro or choose a
defualt value.

##### Panics

```rust
match v {
    Some(num) => num + 10,
    None => panic!("Can't proceed further"), 
}
```

##### Default behaviour

```rust
match v {
    Some(num) => num + 10,
    None => num + 42, // For whatever reason
}
```

## Result Definition

```rust
enum Result<T,E> {
    Err(E), // In case of absence of Value, the cause of the error
    Ok(T), // presemce of value, Ok variant tells the computation was successful
}
```
