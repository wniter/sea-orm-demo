// /**
//  * github：https://github.com/tokio-rs/axum
//  *
//  * 先跑一个官方案例：hellowold
//  * cargo build
//  */
/**
 * 通过一个不使用宏(macro free?)的API将请求路由到处理程序
    使用提取器(extractor)对请求进行声明式的解析
    简单和可预测的错误处理模式。
    用最少的模板生成响应。
    充分利用 tower 和 tower-http 的中间件、服务和工具的生态系统
 */
use axum::{routing::get, Router};
#[tokio::main]
async fn main() {
    // our router
    let app = Router::new()
    //对 GET/ 的请求响应是 200 OK，其中正文是 Hello, World！。任何其他请求将导致 404 Not Found 响应。
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(foo_bar));
    // run it with hyper on localhost:3000
    //启动点
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
// which calls one of these handlers
async fn root() -> String {
    String::from("hello axum")
}
async fn get_foo() -> String {
    String::from("get:foo")
}
async fn post_foo() -> String {
    String::from("post:foo")
}
async fn foo_bar() -> String {
    String::from("foo:bar")
}

// use axum::{
//     http::StatusCode,
//     response::IntoResponse,
//     routing::{get, post},
//     Json, Router,
// };

// use serde::{Deserialize, Serialize};
// use serde_derive::{Serialize, Deserialize};
// use std::net::SocketAddr;

// #[tokio::main]
// async fn main() {
//     // initialize tracing
//     // tracing_subscriber::fmt::init();

//     // build our application with a route
//     let app = Router::new()
//         // `GET /` goes to `root`
//         .route("/", get(root))
//         // `POST /users` goes to `create_user`
//         .route("/users", post(create_user));

//     // run our app with hyper
//     // `axum::Server` is a re-export of `hyper::Server`
//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//     // tracing::debug!("listening on {}", addr);
//     axum::Server::bind(&addr)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }

// // basic handler that responds with a static string
// async fn root() -> &'static str {
//     "Hello, World!"
// }

// async fn create_user(
//     // this argument tells axum to parse the request body
//     // as JSON into a `CreateUser` type
//     Json(payload): Json<CreateUser>,
// ) -> impl IntoResponse {
//     // insert your application logic here
//     let user = User {
//         id: 1337,
//         username: payload.username,
//     };

//     // this will be converted into a JSON response
//     // with a status code of `201 Created`
//     (StatusCode::CREATED, Json(user))
// }

// // the input to our `create_user` handler
// #[derive(Deserialize)]
// struct CreateUser {
//     username: String,
// }

// // the output to our `create_user` handler
// #[derive(Serialize)]
// struct User {
//     id: u64,
//     username: String,
// }
