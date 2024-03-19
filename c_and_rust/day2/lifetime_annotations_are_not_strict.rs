#[derive(Debug)]
struct I(i32);
fn print_multi<'a, 'b>(x: &'a I, y: &'b I) {
    println!("`print_multi`: x is {:?}, y is {:?}", x, y);
}

fn print_single<'a>(x: &'a I, y: &'a I) {
    println!("`print_single`: x is {:?}, y is {:?}", x, y);
}

fn main() {
    let x; // Declare `x` outside the inner scope.
    {
        let a = I(10);
        x = &a; // `x` borrows `a` here.
        
        {
            let b = I(20);
            let y = &b; // `y` borrows `b` here.
            // This call is valid because `print_multi` can accept references with different lifetimes.
            print_multi(x, y);
            print_single(x, y); // x and has different lifetime, yet no error
        } // `b` goes out of scope here, so `y`'s lifetime ends.

    } // `a` goes out of scope here, so `x`'s lifetime ends.
}