use std::fmt;
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Custom ({}, {})", self.x, self.y)
    }
}
// secp256k1

// Float is not used, because it is not accurate
// IEEE standard

// Behaviour

// Define encapsulation in rust?

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point {x, y}
    }

    fn default() -> Point {
        Point {x: 0.0, y: 0.0}
    }
}

trait Perimeter {
    fn perimeter(&self) -> f64;
}

impl Perimeter for Circle {
    fn perimeter(&self) -> f64 {
        3.14 * 2.0 * self.radius
    }
}

impl Perimeter for Rectangle {
    fn perimeter(&self) -> f64 {
        (self.length() + self.width()) * 2.0
    }
}

// Whether Circle and Rectangle have same size?
fn display_perimeter<T>(shape: T) where T: Perimeter {
    // monomorphization
    println!("Perimeter of the shape is {}", shape.perimeter());
} 
// Rust ensures memory safety and type safety

// display_perimeter_circle
// display_perimeter_rectangle

// dyn

fn display_perimeters_specific<T>(shapes: Vec<T>) where  T: Perimeter {
    for shape in shapes {
        println!("{}", shape.perimeter());
    }
}

fn display_perimeters_generic(shapes: Vec<&dyn Perimeter>) {
    for shape in shapes {
        println!("{}", shape.perimeter());
    }
}

#[derive(Debug, Clone, Copy)]
struct Circle {
    center: Point,
    radius: f64
}

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    south_west: Point,
    north_east: Point,    
}

impl Rectangle {
    fn length(&self) -> f64 {
        self.north_east.x - self.south_west.x
    }
    fn width(&self) -> f64 {
        self.north_east.y - self.south_west.y
    }

    pub fn new(south_west: Point, north_east: Point) -> Rectangle {
        if south_west.x > north_east.x || south_west.y > north_east.y {
            panic!("Can't create rectangle");
        }
        Rectangle {south_west, north_east}
    }
}

// Polymorphism by traits

fn main() {
    // let p = Point {x: 12, y: 23};

    // let x0 = 45;
    // let y = 98;
    // let p1 = Point {x: x0, y}; 

    // println!("{:?}", p);
    // println!("{}", p1);
    // Before compilation the macros are expanded into actual rust code
    // That rust code of println! macro expects the type that it tries to print
    // to have a Display trait
    // when it tries to print p, it will call p.fmt(formatter)

    let p_zero = Point {x: 0.0, y: 0.0};
    let p_one = Point {x: 1.0, y: 1.0};

    let rectangle = Rectangle {south_west: p_one, north_east: p_zero};

    println!("{}", rectangle.length());

    display_perimeter(rectangle);
    let circle = Circle {center: p_one, radius: 5.6};

    display_perimeter(circle);

    let shapes: Vec<&dyn Perimeter> = vec![&rectangle, &circle];
    display_perimeters_generic(shapes);

    // display_perimeters(shapes);

    let circles: Vec<Circle> = vec![circle, circle];
    let rectanlges: Vec<Rectangle> = vec![rectangle, rectangle];
    display_perimeters_specific(circles);
    display_perimeters_specific(rectanlges);

    // Vec<Perimeter>
}