struct I<'a> {
    i: &'a i32
}

fn f(s: &mut I) {
    let x = 1;
    s.i = &x;
}

fn main() {
    let mut s = I { i: &0 };
    f(&mut s);
    println!("{}", *s.i);
}