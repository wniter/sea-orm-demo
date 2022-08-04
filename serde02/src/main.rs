// pub mod examples::StructsAndEnums::{W};
/**
 * 注意:rust项目不能建议用驼峰法命名，会包黄。
 *
 * 如何打开在同级目录下rs文件中的结构体
 * 1.在文件下定义mod xxx.rs,然后在同级文件下mod 文件名
 * 2.使用 use::文件名::xx.rs。
 */
mod examples;
use examples::user::User;
use examples::StructAndEnums;
enum E {
    W { a: i32, b: i32 },
    X(i32, i32),
    Y(i32),
    Z,
}

fn main() {
    let w = StructAndEnums::W { a: 0, b: 0 }; // Represented as `{"a":0,"b":0}`
    let x = StructAndEnums::X(0, 0); // Represented as `[0,0]`
    //这个为什么
    // let serialized = serde_json::to_string(&w).unwrap();
    
    // let w = E::W { a: 0, b: 0 }; // Represented as `{"W":{"a":0,"b":0}}`
    // let x = E::X(0, 0); // Represented as `{"X":[0,0]}`
    // let y = E::Y(0); // Represented as `{"Y":0}`
    // let z = E::Z; // Represented as `"Z"`
}
