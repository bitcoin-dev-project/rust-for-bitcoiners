struct Point {
    x: i32,
    y: i32,
}
enum Shape {
    Circle(Point, i32),
    Square(Point, i32),
    Rectangle(Point, Point),
}

fn area(shape: Shape) -> i32 {
    match shape {
        Shape::Circle(_, r) => 3 * r * r,
        Shape::Square(_, a) => a * a,
        Shape::Rectangle(_, _) => todo!(),
    }
}

enum Color {
    Orange,
    Yellow,
    Green,
}

fn display_color(color: Color) {
    // something
}

// Discuss about the pros and cons of pattern matching with enums in Rust

fn main() {
    display_color(Color::Orange);
}
