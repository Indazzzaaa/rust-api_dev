#![allow(unused)]

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let base_url = "http://localhost:8080";
    let hc = httpc_test::new_client(base_url)?;

    hc.do_get("/hello?name=sumant").await?.print().await?;
    Ok(())
}
