# Hex encoding

## What does Hex stands for?

The relationship between "hex" and the number 16 is rooted in the fact that "hex" is a prefix derived from the Greek word "hexa," meaning six, and "deca," meaning ten. Combined, these form "hexadeca," which relates to the number sixteen. This is commonly shortened to "hexa-" in various contexts, most notably in "hexadecimal."

The hexadecimal system is a base-16 number system used extensively in computing and digital systems. It uses sixteen distinct symbols, 0-9 to represent values zero to nine, and A-F to represent values ten to fifteen. This system is particularly useful in computing because it aligns neatly with the binary system, where each hexadecimal digit corresponds directly to four binary digits (bits), making it simpler to read and write binary values.

More on why it is used in bitcoin can be found [here](https://learn.saylor.org/mod/book/view.php?id=54971&chapterid=40654).

**Example**

If you have an array of bytes as `[45, 129, 3]` it's equivalent hex encoding is
`2d8103`.
The number `45` is `2d` in base 16, because `2*16 + 13 = 45`. Similarly you can
derive for other numbers.

## How to work with hex strings in Rust?

As a systems programmer we have to be aware of how much memory our program uses.
It requires 24 bits to store this array `[45, 129, 3]` of bytes in rust.

```rust
let bytes: [u8] = [45, 129, 3]; // 3 * 8 = 24 bits
```

The string representation `2d8103` however takes 48 bytes to be stored in rust,

```rust
let string: &str = "2d8103";
```

because internally rust allocates a byte for each character and there are 6 characters in total.
So we have an interesting relationship here.

```
if hex_string = hex_string_representation(bytes) then
    length(hex_string) == 2 * length(bytes)
```

So Satoshi chose to use a message format which increases the load on the bandwidth of the bitcoin
network for ease of readability.

## Hex string to bytes

One has to remember that not every string is a valid hex string. Because in hex string format
only characters of `[0-9]` and `[a-f]` are permitted. A single character in a hex string represents
4 bits, so two characters has to be clubbed together to form a single byte.
The following code explains how it's done.

```rust
fn hex_to_bytes(hex_string: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
    (0..hex_string.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex_string[i..i + 2], 16))
        .collect()
}
```

## Bytes to Hex string

Any sequence of bytes has a valid hex format. Becaue every byte will be divided into two
`4 bits` part and each of them will have corresponding hex character.
4 bits can represent 16 symbols especially `[0-15]` which are represented using `[0-9]` and `[a-f]`
as hex strings.

The following code explains how it's done.

```rust
fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{:02X}", byte)).collect()
}
```
