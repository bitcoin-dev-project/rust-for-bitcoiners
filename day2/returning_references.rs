fn f<'a, 'b> (x: &'a i32, y: &'b i32) -> &'b i32 {
    y
}

fn main() {
    let x = 1;
    let y = 2;
    let r = f(&x, &y);
    println!("{}", r);
}