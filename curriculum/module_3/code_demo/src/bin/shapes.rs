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

fn main() {
    println!("origin = {:?}", Point::ORIGIN);

    let point = Point {x: 1, y: 2};

    let scaled_point = point.scale_x(6);

    let point = Point {x: 1, y: 2};

    let scaled_point1 = Point::scale_x(point, 6);  

    assert_eq!(scaled_point, scaled_point1);

}
