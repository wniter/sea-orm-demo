/**
 * serde 是rust的一个io序列化框架，先跑一个官网的例子
 * 链接：https://crates.io/crates/serde/1.0.142
 * 文档链接：
 * https://serde.rs/attr-flatten.html
 */

use serde_derive::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
}