// Option and Result
// Null pointer exceptions

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn f(x: i32) -> i32 {
    2 * x
}

fn g(x: i32) -> i32 {
    x - 4
}

fn divide_5(x: i32) -> Option<f64> {
    if x == 0 {
        None
    } else {
        Some(5.0 / (x as f64))
    }
}

fn main() {
    /**
     * x = object(); // Object may or may not exist
     * x.some(); // This might panic at runtime
     */

    let ex = Some(5);
    println!("{:?}", ex);

    let five = match ex {
        Some(v) => v,
        None => 0,
    };
    println!("{}", five);

    let none: Option<i32> = None;
    let zero = none.unwrap_or_default();
    assert_eq!(0, zero);

    let some_p: Option<Point> = Some(Point {
        x: 0,
        y: 0
    });
    let d_p = some_p.unwrap_or_default();
    assert_eq!(some_p.unwrap(), d_p);


    let num = Some(5);
    // You want to apply f and g to this num
    let num_f_g = num.and_then(|x| Some(f(x))).and_then(|x| Some(g(x)));

    assert_eq!(Some(6), num_f_g);
    
}
