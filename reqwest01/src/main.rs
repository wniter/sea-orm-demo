/**
 * github链接
 * https://github.com/seanmonstar/reqwest
 * 先跑一个example
 * An ergonomic, batteries-included HTTP Client for Rust.

Plain bodies, JSON, urlencoded, multipart
Customizable redirect policy
HTTP Proxies
HTTPS via system-native TLS (or optionally, rustls)
Cookie Store
WASM
Changelog
 */
use std::collections::HashMap;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // println!("Hello, world!");
       let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
