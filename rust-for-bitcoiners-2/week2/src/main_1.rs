use std::fmt;

#[derive(Debug, Clone, Copy)] // derive the Debug trait for the Point type
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, Copy)]
struct Circle {
    p: Point,
    r: f32,
}

#[derive(Debug, Clone, Copy)]
struct Recatangle {
    south_west: Point,
    north_east: Point,
}

impl Recatangle {
    fn height(&self) -> f32 {
        self.north_east.y - self.south_west.y
    }
}

impl Area for Recatangle {
    fn area(&self) -> f32 {
        let height = self.north_east.y - self.south_west.y;
        let width = self.north_east.x - self.south_west.x;

        height * width
    }
}

trait Area {
    fn area(&self) -> f32;
}

impl Area for Circle {
    fn area(&self) -> f32 {
        3.14 * self.r * self.r
    }
}

impl Area for Point {
    fn area(&self) -> f32 {
        0.0
    }
}
// IEEE floating point standard

impl Point {
    fn new_y0(x: f32) -> Point {
        Point {x, y: 0.0}
    }
    
    fn default() -> Point {
        Point {x: 0.0, y: 0.0}
    }

    fn new(x: f32, y: f32) -> Point {
        Point {x, y}
    }
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PPPP ({}, {})", self.x, self.y)
    }
}

fn display_area<T>(shape: T) where T: Area {
    // monomorphization
    println!("Area of the shape is {}", shape.area());
}

fn demo() {
    let circle = Circle{};
    let rect = Rect{...};

    display_area(circle);
    /*  compiler will genrate a function 
    fn display_area_circle(shape: Circle) {..}
    */
    display_area(rect);
    /*  compiler will genrate a function 
    fn display_area_rectangle(shape: Rectangle) {..}
    */

    display_area_dyn(cirlce);
    // It does not generate different code for different types
    // But how it will differentiate between a Circle and a Rectangle?
    // Rust is going to create a new data structure (type) which is called a Virtual table
    // for the Area trait
}

fn display_area_dyn(shape: &dyn Area) {
    // we have only one code generation for all the types
    println!("dyn Area of the shape is {}", shape.area());
    // Virtual method table lookup happens, we get the associated function pointer
    // go to that function and execute the corresponding code
}

fn display_areas_specific<T>(shapes: Vec<T>) where T: Area {
    for shape in shapes {
        display_area(shape);
    }
}
// Why Vec<Area> is not a valid type?
// Vec<T: Area>

fn display_areas_polymorphic(shapes: Vec<&dyn Area>) {
    for shape in shapes {
        display_area_dyn(shape);
    }
}

// display_area_rectangle
// display_area_circle

fn main() {
    let p = Point {y:1.0, x:2.3};
    println!("{:?}", p);

    let x = 5.0;
    let y = -43.5;
    let p1 = Point {x, y};
    println!("{}", p1);

    let p2 = Point::new_y0(60.5);

    let circle = Circle {p: Point::default(), r: 6.1};

    let rectangle = Recatangle {south_west: Point::new(1.0, 1.0), north_east: Point::new(0.0, 0.0)};

    println!("height is {}", rectangle.height());

    let shapes: Vec<Circle> = vec![circle, circle];
    display_areas_specific(shapes);

    let shapes: Vec<&dyn Area> = vec![&circle, &rectangle, &p2];
    display_areas_polymorphic(shapes);

    display_area_dyn(&rectangle);
    display_area_dyn(&circle);

    display_area(rectangle);

    display_area(circle);
}
