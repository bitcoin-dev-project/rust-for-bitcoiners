What is an array?

At a high level an array is a mapping from set (`S` = [0,1, ..., n-1]) to an arbitrary set `V`.
In rust this `V` could be a struct or an enum.

In hash map that type `S` could be an arbitrary set `K`.
In rust this `K` could be any struct or an enum which implements these `Eq, Hash, PartialEq` traits.

Eq states that every x, y belongs to a set A satisfies (x == y and y == x) or x != y.
PartialEq means the above condition need not apply to all the elements.
Example take f64, consider `inf` which does not satify `inf` == `inf` and `inf` != `inf`

## What is the purpose of HashMap datastructure?

It's purpose is to keep track of all key value pairs that the user inserts,
where the key could be a value of arbitrary type in an array data structure.

## What is the purpose of Hash trait?

It is to define a mapping from set `K` to `S`.

```rust
let x = "asbce";
let y = 58;

let hash_map: [Vec<&str>; 1000] = [""; 1000];

if hash(x) == y {
    // then you will do the following
    /// hasp_map.insert("random")
    hash_map[58] = "random";
}
```

## Why HashMap is used to represent graph data structure?


