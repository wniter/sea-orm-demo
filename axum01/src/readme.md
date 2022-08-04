使用示例
axum 的 “hello world” 是这样的:

use axum::prelude::*;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = route("/", get(root));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
对 GET/ 的请求响应是 200 OK，其中正文是 Hello, World！。任何其他请求将导致 404 Not Found 响应。

提取器
请求可以使用 “提取器/extractor” 进行声明式的解析。提取器是一个实现了 FromRequest 的类型。提取器可以作为处理程序的参数，如果请求的URI匹配，就会运行。

例如，Json 是一个提取器，它消耗请求主体并将其解析为JSON:

use axum::{prelude::*, extract::Json};
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

async fn create_user(Json(payload): Json<CreateUser>) {
    // `payload` is a `CreateUser`
}

let app = route("/users", post(create_user));
axum 提供了许多有用的提取器，例如:

Bytes, String, Body, 和 BodyStream 用于获取请求正文
Method, HeaderMap, 和 Uri 用于获取请求的特定部分
Form, Query, UrlParams, 和 UrlParamsMap 用于更高级别的请求解析
Extension 用于跨处理程序共享状态的扩展
Request<hyper::Body> 如果你想完全控制
Result<T, E> and Option<T> 使提取器成为可选
你也可以通过实现 FromRequest 来定义你自己的提取器。

构建响应
处理程序可以返回任何实现了 IntoResponse 的东西，它将被自动转换为响应:

use http::StatusCode;
use axum::response::{Html, Json};
use serde_json::{json, Value};

// We've already seen returning &'static str
async fn text() -> &'static str {
    "Hello, World!"
}

// String works too
async fn string() -> String {
    "Hello, World!".to_string()
}

// Returning a tuple of `StatusCode` and another `IntoResponse` will
// change the status code
async fn not_found() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "not found")
}

// `Html` gives a content-type of `text/html`
async fn html() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

// `Json` gives a content-type of `application/json` and works with any type
// that implements `serde::Serialize`
async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}
这意味着在实践中，你很少需要建立你自己的响应。你也可以实现 IntoResponse 来创建你自己的特定领域响应。

路由
可以使用一个简单的 DSL 来组合多个路由。

use axum::prelude::*;

let app = route("/", get(root))
    .route("/users", get(list_users).post(create_user))
    .route("/users/:id", get(show_user).delete(delete_user));
中间件
axum 支持来自 tower 和 tower-http 的中间件。

use axum::prelude::*;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};
use tower::ServiceBuilder;
use std::time::Duration;

let middleware_stack = ServiceBuilder::new()
    // timeout all requests after 10 seconds
    .timeout(Duration::from_secs(10))
    // add high level tracing of requests and responses
    .layer(TraceLayer::new_for_http())
    // compression responses
    .layer(CompressionLayer::new())
    // convert the `ServiceBuilder` into a `tower::Layer`
    .into_inner();

let app = route("/", get(|| async { "Hello, World!" }))
    // wrap our application in the middleware stack
    .layer(middleware_stack);
这个功能很关键，因为它允许我们只写一次中间件，并在不同的应用中分享它们。例如，axum 不需要提供自己的 tracing/logging 中间件，可以直接使用来自 tower-http 的 TraceLayer 。同样的中间件也可以用于用 tonic 制作的客户端或服务器。

路由到任何 tower::Service

axum 也可以将请求路由到任何 tower 服务。可以是你用 service_fn 编写的服务，也可以是来自其他 crate 的东西，比如来自 tower-http 的ServeFile：

use axum::{service, prelude::*};
use http::Response;
use std::convert::Infallible;
use tower::{service_fn, BoxError};
use tower_http::services::ServeFile;

let app = route(
    // Any request to `/` goes to a some `Service`
    "/",
    service::any(service_fn(|_: Request<Body>| async {
        let res = Response::new(Body::from("Hi from `GET /`"));
        Ok::<_, Infallible>(res)
    }))
).route(
    // GET `/static/Cargo.toml` goes to a service from tower-http
    "/static/Cargo.toml",
    service::get(ServeFile::new("Cargo.toml"))
);