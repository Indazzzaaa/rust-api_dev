#![allow(unused)]

use std::net::SocketAddr;

use axum::{response::Html, routing::get, serve::Listener, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("Hello <strong> World!! </strong>") }),
    );

    // #region -- Start Server
    let addr = "127.0.0.1:8080";
    let listner = TcpListener::bind(addr).await.unwrap();
    println!("->> LISTENING on {addr}\n");

    axum::serve(listner,routes_hello.into_make_service()).await.unwrap();

    // #endregion
}
