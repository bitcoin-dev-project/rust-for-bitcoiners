#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    const ORIGIN: Point = Point { x: 0, y: 0};

    // self -> point: Point
    fn scale_x(self, scaling_factor: i32) -> Point {
        Point { x: self.x * scaling_factor, y: self.y }
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

struct Square {
    centre: Point,
    side: u32,
}

trait Shape {
    fn area(&self) -> u32;
    fn perimeter(&self) -> u32;
}

impl Shape for Circle {
    fn area(&self) -> u32 {
        3 * self.radius * self.radius
    }

    fn perimeter(&self) -> u32 {
        2 * 3 * self.radius
    }
}

impl Shape for Square {
    fn area(&self) -> u32 {
        self.side * self.side
    }

    fn perimeter(&self) -> u32 {
        4 * self.side
    }
}

impl Shape for Rectangle {
    fn area(&self) -> u32 {
        todo!()
    }

    fn perimeter(&self) -> u32 {
        todo!()
    }
}
// Take arbitrary shape and do something with it,
// Find it's area or perimeter

fn display_props_of_shape<T>(shape: T) where T: Shape {
    println!("area = {}, perimeter = {}", shape.area(), shape.perimeter());
} 

// display_props_of_shape_circle(..) {..}
// display_props_of_shape_square(..) {..}

fn main() {
    println!("origin = {:?}", Point::ORIGIN);

    let point = Point {x: 1, y: 2};

    let scaled_point = point.scale_x(6);

    let point = Point {x: 1, y: 2};

    let scaled_point1 = Point::scale_x(point, 6);  

    assert_eq!(scaled_point, scaled_point1);

}
