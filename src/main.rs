#![allow(unused)]

use axum::{
    Router,
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
    serve::Listener,
};
use serde::Deserialize;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/{name}", get(hanlder_hello2));

    // region:    --- Start Server
    let addr = "127.0.0.1:8080";
    let listner = TcpListener::bind(addr).await.unwrap();
    println!("->> LISTENING on {addr}\n");

    axum::serve(listner, routes_hello.into_make_service())
        .await
        .unwrap();
    // endregion: --- Start Server
}

// region: --- Handler Hello

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
// endregion: --- Handler Hello
