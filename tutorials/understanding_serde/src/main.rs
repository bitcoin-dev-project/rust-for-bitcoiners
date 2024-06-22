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
