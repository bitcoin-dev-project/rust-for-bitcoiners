#![allow(unused)]
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn print_sized<T: Sized + std::fmt::Debug>(t: T) {
    println!("{:?}", t);
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Self { x: self.x.clone(), y: self.y.clone() }
    }
}

impl Copy for Point {}


#[derive(Clone)]
struct Account {
    value: String
}

fn main() {
    let p = Point { x: 5, y: -2 };
    println!("x = {}, y = {}", p.x, p.y);
    let p1 = p;
    println!("x = {}, y = {}", p.x, p.y);

    print_sized(p);
}