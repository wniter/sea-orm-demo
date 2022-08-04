use serde_derive::{Deserialize, Serialize};


//在结构体上加#[derive(Serialize, Deserialize)]可以直接进行序列化
// #[derive(Serialize, Deserialize)]
// enum Message {
//     Request { id: String, method: String, params: Params },
//     Response { id: String, result: Value },
// }
// {"type": "Request", "id": "...", "method": "...", "params": {...}}

//相邻标记
// #[derive(Serialize, Deserialize)]
// #[serde(tag = "t", content = "c")]
// enum Block {
//     Para(Vec<Inline>),
//     Str(String),
// }
// {"t": "Para", "c": [{...}, {...}]}
// {"t": "Str", "c": "the string"}


//Untagged
// #[derive(Serialize, Deserialize)]
// #[serde(untagged)]
// enum Message {
//     Request { id: String, method: String, params: Params },
//     Response { id: String, result: Value },
// }
// {"id": "...", "method": "...", "params": {...}}