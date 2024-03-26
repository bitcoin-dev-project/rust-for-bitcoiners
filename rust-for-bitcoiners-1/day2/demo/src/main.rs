#[allow(unused)]

// Demo of enums and pattern matching with method call syntax
#[derive(Debug)]
enum MResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> MResult<T, E> {
    fn ok(value: T) -> Self {
        MResult::Ok(value)
    }
    // Function to create an Err variant
    fn err(error: E) -> Self {
        todo!()
    }

    // Method to check if it's an Ok variant
    fn is_ok(&self) -> bool {
        todo!()
    }

    // Method to check if it's an Err variant
    fn is_err(&self) -> bool {
        todo!()
    }

    // Method to unwrap the Ok value, panics if it's an Err
    fn unwrap(self) -> T {
        todo!()
    }

    // Method to unwrap the Err value, panics if it's an Ok
    fn unwrap_err(self) -> E {
        todo!()
    }
}
// self, &self and &mut self

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn lost(self) {
        println!("point {:?} no longer exists", self);
    }

    fn set_x(&mut self, new_x: i32) {
        todo!()
    }

    fn get_y(&self) -> i32 {
        todo!()
    } 
}

struct Rectangle {
    _1: Point,
    _2: Point,
}

struct Circle {
    center: Point,
    radius: u32,
}

struct Square {
    center: Point,
    side: u32,
}

trait Shape {
    fn area(&self) -> u32;
    fn perimeter(&self) -> u32;
}

impl Shape for Square {
    fn area(&self) -> u32 {
        todo!()
    }
    fn perimeter(&self) -> u32 {
        self.side * 4
    }
}

fn print_area_and_perimeter<T: Shape>(shape: T) {
    todo!()
}

// Putting enums and structs together to build Linked List

// Oh oh
enum List<T> {
    Empty,
    Node {
        data: T,
        next: Next<T>,
    }
}

enum Next<T> {
    Address(Box<List<T>>),
    Nil,
}


fn main() {
    println!("{:?}", MResult::<i32, String>::ok(34));

    let p = Point{x: 1, y: 2};
    p.lost();
    // println!("{:?}", p); Hence not Object oriented

}