// f(x,y,z) -> a

fn f(x: i32, y: i32, z: i32) -> i32 {
    x - y + 4 * z
    // return value;
}

fn inc<T>(x: T) -> T
where T: std::ops::Add<Output=T> + From<u8> {
    x + T::from(1u8)
}
// We can't use inc(10i8)

fn main() {
    let res = f(7, -3, 100);
    println!("{res}");

    inc(-4i128);
    inc(3456u32);
    inc(-90i16);
    
    inc(7i8);
}