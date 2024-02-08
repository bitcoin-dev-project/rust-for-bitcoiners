fn f<'a, 'b> (x: &'a i32, y: &'b i32, threshold: i32) -> &'a i32 {
    if *x > threshold {
        x
    } else {
        y
    }
}

fn g(x: &i32) -> &i32 {
    x
}

fn main() {
    let x = 1;
    let y = 2;
    let r = f(&x, &y, 5);
    println!("{}", r);
}