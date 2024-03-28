#[allow(unused)]

// Demo of enums and pattern matching with method call syntax
#[derive(Debug)]
enum MResult<T, E> {
    Ok(T),
    Err(E),
}

struct IntList {
    value: i32,
    next: Box<IntList> // reference, smart pointer and we know it's size
}

// Sum and products

enum Expr {
    Integer(i32), // 256 bits -> [discriminant] [value]
    Add(Box<Expr>, Box<Expr>), // 
    Sub(Box<Expr>, Box<Expr>), // 2
}

fn evaluate_expr(expr: Expr) -> i32 {
    match expr {
        Expr::Integer(value) => value,
        Expr::Add(e1, e2) => evaluate_expr(*e1) + evaluate_expr(*e2),
        Expr::Sub(e1, e2) => evaluate_expr(*e1) - evaluate_expr(*e2),
    }
}

/*
B extends A
C extends A

Which means A could be A or B or C
 */

impl<T> MResult<T, String> {
}

impl<E> MResult<String, E> {
    fn only_for_string_ok(&self) -> String {
        // This method does not have the ownership of the self, so it cannot
        // return the reference as a value to caller
        match self {
            MResult::Ok(value) => value.clone(), // duplicate the value
            MResult::Err(_) => "Error encountered".to_owned(),
        }
    }
}

impl<T, E> MResult<T, E> {
    fn ok(value: T) -> Self {
        // associated function
        MResult::Ok(value)
    }
    // Function to create an Err variant
    fn err(error: E) -> Self {
        MResult::Err(error)
    }

    // Method to check if it's an Ok variant
    fn is_ok(&self) -> bool {
        match self {
            MResult::Ok(_) => true,
            _ => false,
        }
    }

    // Method to check if it's an Err variant
    fn is_err(&self) -> bool {
        todo!()
    }

    // Method to unwrap the Ok value, panics if it's an Err
    fn unwrap(self) -> T {
        // Why in the input I'm taking the complete ownership of self?
        todo!()
    }

    // Method to unwrap the Err value, panics if it's an Ok
    fn unwrap_err(self) -> E {
        match self {
            MResult::Err(err) => err,
            _ => panic!("Not an error"),
        }
    }
}
// self, &self and &mut self

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    // Associated function
    fn new(x: i32, y: i32) -> Point {
        Point {x, y}
    }

    // Associated method which takes the ownership of the Point value
    fn lost(self) {
        println!("point {:?} no longer exists", self);
    }

    // In OOP you only have the &mut self variant
    // Associated method which takes the mutable reference of the Point value
    fn set_x(&mut self, new_x: i32) {
        self.x = new_x;
    }

    // Associated method which takes the read only reference of the Point value
    fn get_y(&self) -> i32 {
        self.y
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
        self.side * self.side
    }
    fn perimeter(&self) -> u32 {
        self.side * 4
    }
}

impl Shape for Circle {
    fn area(&self) -> u32 {
        3 * self.radius * self.radius
    }

    fn perimeter(&self) -> u32 {
        2 * 3 * self.radius
    }
}

// Crazy things

// u32 is considered as a foreign type because it is not defined within this file context
impl Shape for u32 {
    fn area(&self) -> u32 {
        *self * *self * 10 // Insane
    }
    fn perimeter(&self) -> u32 {
        *self * 4
    }
}

/*
NOT Allowed
impl u32 {
    fn go_brr(self) -> Self {
        self * 1000
    }
}
*/

fn print_area<T: Shape>(shape: T) {
    println!("Area of the shape is {}", shape.area());
}

// for the above function the compiler will create two copies of function one for Square and the other
// for Circle similar to what is shown below, this is one example of Zero Cost Abstraction in Rust

/*
fn print_area_square(square: Square) {

}
fn print_area_circle(circle: Circle) {

}
 */

// Dynamic dispatch using Vtables, this is executed during runtime
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
// 1 -> 2 -> 3 == List::Node {1, Node {2, Node 3, List::Empty}}.push(0) == 0 -> 1 -> 2 -> 3

impl <T> List<T> {
    fn empty() -> List<T> {
        List::Empty
    }
    fn singleton(value: T) -> List<T> {
        List::Node { data: value, next: Box::new(List::Empty) }
    }

    fn prepend(self: List<T>, value: T) -> List<T> {
        List::Node{data: value, next: Box::new(self)}
    }
    
    // Iterators need to be able to iterate through the structure one by one
    // The List that we defined above do not have that feature.
    // So create a new struct ListIterator which has a single variable whose only role
    // is to point to nodes of the List. We implement iterator for ListIterator.
    // The below function gives us the ListIterator which will enable use to iterate through
    // the List one by one.
    pub fn iter<'a>(&'a self) -> ListIterator<'a, T> {
        ListIterator { current: self }
    }
}

// Why lifetime parameter here needed is because the `current` field has to point
// to node of the List and the compiler has to make sure that the reference held by
// current is valid, i.e., the List lives longer than the ListIterator.
pub struct ListIterator<'a, T> {
    current: &'a List<T>,
}

impl<'a, T> Iterator for ListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
         match self.current {
            List::Empty => None,
            List::Node { data, next } => {
                self.current = next; // Move the current to the next block in List
                Some(data) // return the current value
            },
        }
    }
}

fn main() {
    // println!("{:?}", MResult::<i32, String>::ok(34));

    // let res: MResult<i32, String> = MResult::ok(94);
    // println!("{}", res.is_ok());

    // let res: MResult<i32, String> = MResult::err("String".to_owned());
    // println!("{}", res.is_ok());

    // let p = Point{x: 1, y: 2};
    // p.lost();
    // let p = Point{x: 1, y: 2};
    // // println!("{:?}", p); Hence not Object oriented
    // println!("{:?}", p.get_y());

    let square = Square {
        center: Point{x: 0, y : 2},
        side: 5,
    };
    let circle = Circle {
        center: Point{x: 0, y : 2},
        radius: 50,
    };

    print_area(square);
    print_area(circle);

    let items = vec![1,2,3,4];
    let mut items_1 = Vec::new();
    for i in 1..5 {
        items_1.push(i);
    }
    assert_eq!(items, items_1);
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

    #[test]
    fn test_area_u32() {
        assert_eq!(5.area(), 250);
    }
}