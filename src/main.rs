#![allow(unused)]

// region: -- deps

use axum::{
    Router, ServiceExt,
    extract::{Path, Query},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, get_service},
    serve::Listener,
};
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
// endregion: -- deps

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello())
        .fallback_service(get_service(ServeDir::new("public"))); // to serve static data

    // region:    --- Start Server
    let addr = "127.0.0.1:8080";
    let listner = TcpListener::bind(addr).await.unwrap();
    println!("->> LISTENING on {addr}\n");

    axum::serve(listner, routes_all.into_make_service())
        .await
        .unwrap();
    // endregion: --- Start Server
}

// region: --- Routes Hello
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/{name}", get(hanlder_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// e.g., '/hello?name=Sumant'

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");

    Html(format!("Hello <strong> {name} </strong>"))
}

// e.g., '/hello2/Sumant'
async fn hanlder_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");

    Html(format!("Hello <strong> {name} </strong>"))
}
// endregion: --- Routes Hello
