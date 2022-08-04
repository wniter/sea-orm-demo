//创建一个类或者枚举进行转化json

#[derive(Debug)]
pub struct W {
    pub a: i32,
    pub b: i32,
}
 pub struct X(pub i32, pub i32);
// let w = W { a: 0, b: 0 }; // Represented as `{"a":0,"b":0}`
#[derive(Debug)]
struct Y {
    name: String,
    age: i32,
}
// impl W {
//     pub fn new() -> W {
//         W{}
//     }

//     pub fn new(a: i32, b: i32) -> W {
//         return W {
//             this.a =a,
//             this.b =b
//         }
//     }
// }
