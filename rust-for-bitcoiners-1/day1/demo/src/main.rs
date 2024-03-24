#[allow(unused)]

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let x = 5;
    let mut y = 6;
    y += 50;

    // With let x is a completely new variable, which refers to a new memory location
    // variabel shadowing, similar to that we see in python
    let x = (1,2); // String literal

    // Pointer issues are illegal memory access, this can happen in single threaded code itself
    // Data races

    // let x = 5;
    // println!("{x}");

    // let z = x;
    // println!("{x}");

    // let p1 = Point{x: 1,y: 2}; // Stored in stack
    // println!("{:?}", p1);
    // // let p2 = Point{x: p1.x, y: p1.y};
    // let p2 = p1;
    // println!("{:?}", p1);

    // int x = 10;
    // i8, i16, i128, u8, ..., u128
    // ADD R1 R2

    let mut x: u8 = 122;
    // 8 -> 2^8 = 256, -128 to 127, -2^7 to 2^7 - 1
    // i128 -> -2^127 to 2^127 - 1
    // x = 128
    // x += 1; Constant folding
    f(x);
    f(123);

    let mut a: i16 = 65;
    let mut b: i16 = -231;
    swap(&mut a, &mut b);
    println!("a = {a}, b = {b}");

    let mut a: i64 = 65;
    let mut b: i64 = -231;
    swap(&mut a, &mut b);
    println!("a = {a}, b = {b}");

}

fn f(x: u8) {
    let y = x.saturating_add(1);
    println!("{:?}", y);
}

fn swap<T: Copy>(a: &mut T, b: &mut T) {
    // values between a and b should be swapped
    let t = *a; // Copying of value
    *a = *b;
    *b = t;
}