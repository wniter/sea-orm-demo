/**
 * hyper是rust的它使用两个库：Tokio的事件循环(用于发出非阻塞请求)和Futures. 下面是一个基于Hyper的示例，其灵感主要来自其文档中的示例。
 * A fast and correct HTTP implementation for Rust.
 * hyper 是 Rust 实现的 HTTP 库。hyper 同时支持 HTTP/1 和 HTTP/2，并且同时提供 client 与 server API。

hyper 性能好，偏底层，而且面向 async 设计，应用广泛，已成为 Rust 网络程序生态的重要基石之一。
 知名的 HTTP client reqwest, HTTP server warp 和 axum, Rust 的 gRPC 实现 tonic 等，都使用了 hyper。
 我们不一定会直接使用 hyper，但了解 hyper 对于我们了解 Rust 的网络程序生态，学习设计良好的网络程序，都有好处。
 * https://github.com/hyperium/hyper
 */

use hyper::{Client, Uri};
use std::str;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new();
    let uri = Uri::from_static("http://httpbin.org/ip"); // panic if not valid
    let mut res = client.get(uri).await?;

    println!("status code: {}", res.status());
    for (key, value) in res.headers().iter() {
        println!("{}: {}", key, value.to_str().unwrap())
    }

    let body = res.body_mut();
    let buf = hyper::body::to_bytes(body).await?;
    let content = str::from_utf8(buf.as_ref())?;
    println!("{}", content);

    Ok(())
}