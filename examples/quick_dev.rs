#![allow(unused)]

use anyhow::Result;
use lazy_regex::regex_captures;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let base_url = "http://localhost:8080";
    let hc = httpc_test::new_client(base_url)?;

    // hc.do_get("/test.txt").await?.print().await?;
    // hc.do_get("/hello").await?.print().await?;

    let req_log = hc.do_post(
        "/api/login",
        json!({
            "username" : "demo1",
            "pwd" : "welcome"
        })
    );

    req_log.await?.print().await?;

    // hc.do_get("/hello").await?.print().await?;

    let req_create_ticket = hc.do_post("/api/tickets", json!({"title": "Ticket AAA"}));

    req_create_ticket.await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    // hc.do_delete("/api/ticket/1").await?.print().await?;

    Ok(())
}
