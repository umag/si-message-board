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
    let req_create_message2 = hc.do_post(
        "/api/messages",
        json!({
            "body":"Hello, SI!"
        })
    );
    req_create_message2.await?.print().await?;

    hc.do_delete("/api/messages/0").await?.print().await?;
    hc.do_get("/api/messages").await?.print().await?;
    hc.do_put(
        "/api/messages/1",
        json!({
            "body":"Hello, SI! Updated"
        })
    ).await?.print().await?;
    hc.do_get("/api/messages").await?.print().await?;
    Ok(())
}