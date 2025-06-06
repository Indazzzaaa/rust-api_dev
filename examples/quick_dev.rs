#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let base_url = "http://localhost:8080";
    let hc = httpc_test::new_client(base_url)?;

    // hc.do_get("/test.txt").await?.print().await?;
    hc.do_get("/hello").await?.print().await?;

    let req_log = hc.do_post(
        "/api/login",
        json!({
            "username" : "demo1",
            "pwd" : "welcome"
        }),
    );

    req_log.await?.print().await?;

    hc.do_get("/hello").await?.print().await?;
    Ok(())
}
