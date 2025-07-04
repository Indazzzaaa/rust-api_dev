use axum::{Json, Router, routing::post};
use serde::Deserialize;
use serde_json::{Value, json};
use tower_cookies::{Cookie, Cookies};

use crate::{Error, Result, web};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

// Json<T> is body extractor and it must be last one in arguments
async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // Todo: Implement read db/auth logic.
    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    // Fixme: Implement real auth-token generation/signature.
    cookies.add(Cookie::new(web::ATUH_TOKEN, "user-1.exp.sign"));

    // Create the success body
    let body = Json(json!({
        "result" : {
            "sucess" : true
        }
    }));
    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}
