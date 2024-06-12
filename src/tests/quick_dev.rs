#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]

async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:3000")?;
    
    let req_create_message = hc.do_post(
        "/api/messages",
        json!({
            "body":"Hello, world!"
        })
    );

    req_create_message.await?.print().await?;

    hc.do_get("/api/messages").await?.print().await?;
    Ok(())
}