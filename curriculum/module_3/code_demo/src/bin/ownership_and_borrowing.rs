#[derive(PartialEq, Eq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn scale_x(p: &Point, a: i32) -> Point {
    Point { x: p.x * a, y: p.y }
}

fn scale_x_mut(p: &mut Point, a: i32) {
    p.x = p.x * a;
}

fn main() {
    let p1 = Point { x: 1, y: 1 };
    assert_eq!( Point { x: 5, y: 1 }, scale_x(&p1, 5) );
    println!("{:?}", p1);
    
    let y = &p1;
    let z = &p1;
    assert_eq!( Point { x: 4, y: 1 }, scale_x(z, 4) );

    assert_eq!( Point { x: -4, y: 1 }, scale_x(y, -4) );
    
    let mut p2 = Point { x: 5, y: 2 };
    let m_p2 = &mut p2;
    scale_x_mut(m_p2, -2);
    scale_x_mut(m_p2, -2);
    assert_eq!(p2, Point { x: 20, y: 2 });

}
