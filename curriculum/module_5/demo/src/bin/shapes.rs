#![allow(unused)]

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

/**
 * impl Debug for Point {
 *  Generates the implementation
 * }
 */

impl Point {
    const ORIGIN: Point = Point { x: 0, y: 0};

    // self -> point: Point
    fn scale_x(self, scaling_factor: i32) -> Point {
        Point { x: self.x * scaling_factor, y: self.y }
    } 
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point {x, y}
    }
}

#[derive(Debug)]
struct Circle {
    centre: Point,
    radius: u32,
}

struct Rectangle {
    centre: Point,
    length: u32,
    width: u32,
}

#[derive(Clone, Copy)]
struct Square {
    centre: Point,
    side: u32,
}

trait Shape {
    fn area(&self) -> u64;
    fn perimeter(&self) -> u64;
}

impl Shape for Square {
    fn area(&self) -> u64 {
        self.side as u64 * self.side as u64
    }

    fn perimeter(&self) -> u64 {
        4 * self.side as u64
    }
}

impl Shape for Rectangle {
    fn area(&self) -> u64 {
        self.length as u64 * self.width as u64
    }
    fn perimeter(&self) -> u64 {
        todo!()
    }
}

impl Shape for Circle {
    fn area(&self) -> u64 {
        todo!()
    }
    fn perimeter(&self) -> u64 {
        todo!()
    }
}

fn display_area_and_perimeter<T: Shape>(shape: T) {
    println!("area = {} perimeter = {}", shape.area(), shape.perimeter());
}

fn main() {
    println!("origin = {:?}", Point::ORIGIN);

    let point = Point {x: 1, y: 2};

    let scaled_point = point.scale_x(6);

    let point = Point {x: 1, y: 2};

    let scaled_point1 = Point::scale_x(point, 6);  

    assert_eq!(scaled_point, scaled_point1);

    let square = Square { centre: point, side: 6 };
    display_area_and_perimeter(square);

    Square::area(&square);

}
