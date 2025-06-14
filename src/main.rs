#![allow(unused)]

// region: -- deps

use axum::{
    Router,
    ServiceExt,
    extract::{ Path, Query },
    http::StatusCode,
    middleware,
    response::{ Html, IntoResponse, Response },
    routing::{ get, get_service },
    serve::Listener,
};
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_cookies::{ CookieManager, CookieManagerLayer };
use tower_http::services::ServeDir;

mod ctx;
mod error;
mod model;
mod web;

use crate::model::ModelController;

pub use self::error::{ Error, Result };
// endregion: -- deps

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize ModelController
    let mc = ModelController::new().await?;

    let routes_apis = web::routes_tickets
        ::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth)); // only this route will be impacted from this middleware.

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new()) // Layer get executed from bottom to top
        .fallback_service(get_service(ServeDir::new("public"))); // to serve static data

    // region:    --- Start Server
    let addr = "127.0.0.1:8080";
    let listner = TcpListener::bind(addr).await.unwrap();
    println!("->> LISTENING on {addr}\n");

    axum::serve(listner, routes_all.into_make_service()).await.unwrap();
    // endregion: --- Start Server

    Ok(())
}

// region: --- Routes Hello
fn routes_hello() -> Router {
    Router::new().route("/hello", get(handler_hello)).route("/hello2/{name}", get(hanlder_hello2))
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

// region: -- Response mapper

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:12} - main_response_mapper", "RES_MAPPER");

    // extra line for showing better request flow in debug logs
    println!();
    res
}

// endregion: -- Response mapper
