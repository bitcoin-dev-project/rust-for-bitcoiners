#[allow(unused)]

// Demo of enums and pattern matching with method call syntax
#[derive(Debug)]
enum MResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T> MResult<T, ()> {
}

impl<i32, E> MResult<i32, E> {
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

fn print_area<T: Shape>(shape: T) {
    todo!()
}

fn print_perimeter(shape: &dyn Shape) {
    println!("Perimeter: {}", shape.perimeter());
}

// Putting enums and structs together to build Linked List

// Oh oh
#[derive(Debug, PartialEq, Eq)]
enum List<T> {
    Empty,
    Node {
        data: T,
        next: Box<List<T>>
    }
}

impl <T> List<T> {
    fn empty() -> List<T> {
        List::Empty
    }
    fn singleton(value: T) -> List<T> {
        List::Node { data: value, next: Box::new(List::Empty) }
    }
}


fn main() {
    println!("{:?}", MResult::<i32, String>::ok(34));

    let p = Point{x: 1, y: 2};
    p.lost();
    // println!("{:?}", p); Hence not Object oriented

}

// Write tests for List below
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_empty() {
        let list: List<i32> = List::empty();
        assert_eq!(list, List::Empty);
    }

    #[test]
    fn test_list_singleton() {
        let list: List<i32> = List::singleton(1);
        assert_eq!(list, List::Node { data: 1, next: Box::new(List::Empty) });
    }
}