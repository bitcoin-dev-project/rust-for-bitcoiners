#![allow(unused)]
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "sdeawf"))]
struct Point {
    x: i32,
    y: i32,
}

fn demo_point() {
    let p = Point{x: 5, y: -6};
    println!("{:?}", p);
    let serialized_p: String = serde_json::to_string(&p).unwrap();
    println!("{}", serialized_p);
    let deserilized_p: Point = serde_json::from_str(&serialized_p).unwrap();
    println!("{:?}", deserilized_p);
    let stack_str = "acewfwa";
}

enum Expr<'a> {
    Integer(i32),
    Add(&'a Expr<'a>, &'a Expr<'a>),
    Sub(&'a Expr<'a>, &'a Expr<'a>),
}

fn demo_expr() {
    // let add = Expr::Add(Box::new(Expr::Integer(43)), Box::new(Expr::Integer(22)));
    // let s_add = serde_json::to_string(&add).unwrap();
    // println!("{:?}", s_add);
    let one = Expr::Integer(1);
    let two = Expr::Integer(2);
    let add = Expr::Add(&one, &two);
}

fn referring_to_lower_stack_frame(expr: &Expr) {
    let sub = Expr::Sub(&Expr::Integer(6), expr);
}

fn main() {
    demo_expr();
    demo_point();
}