#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
struct Point {
    x: i32,
    y: i32,
}
#[automatically_derived]
impl ::core::fmt::Debug for Point {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "Point",
            "x",
            &self.x,
            "y",
            &&self.y,
        )
    }
}
fn print_sized<T: Sized + std::fmt::Debug>(t: T) {
    {
        ::std::io::_print(format_args!("{0:?}\n", t));
    };
}
impl Clone for Point {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}
impl Copy for Point {}
fn main() {
    let p = Point { x: 5, y: -2 };
    {
        ::std::io::_print(format_args!("x = {0}, y = {1}\n", p.x, p.y));
    };
    let p1 = p;
    {
        ::std::io::_print(format_args!("x = {0}, y = {1}\n", p.x, p.y));
    };
    print_sized(p);
}
