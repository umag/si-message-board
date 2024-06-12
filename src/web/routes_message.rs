use axum::extract::{State,Path};
use axum::routing::{delete, post, put};
use axum::{Json, Router};
use serde::de;
use tokio::io::Join;

use crate::model::{ModelController, Message, MessageForCreate};
use crate::Result;


async fn create_message(
    State(mc): State<ModelController>,
    Json(message_fc): Json<MessageForCreate>,
)-> Result<Json<Message>> {
    println!("->> {:<12} - create_message", "HANDLER");

    let message = mc.create_message(message_fc).await?;
    Ok(Json(message))
}

async fn list_messages(
    State(mc): State<ModelController>,
)-> Result<Json<Vec<Message>>> {
    println!("->> {:<12} - list_messages", "HANDLER");

    let messages = mc.list_messages().await?;
    Ok(Json(messages))
}

async fn delete_message(
    State(mc): State<ModelController>,
    Path(id): Path<u64>,
)-> Result<Json<Message>> {
    println!("->> {:<15} - delete_message", "HANDLER");

    let message = mc.delete_message(id).await?;
    Ok(Json(message))
}

async fn update_message(
    State(mc): State<ModelController>,
    Path(id): Path<u64>,
    Json(message_fc): Json<MessageForCreate>,
)-> Result<Json<Message>> {
    println!("->> {:<15} - update_message", "HANDLER");

    let message = mc.update_message(id, message_fc).await?;
    Ok(Json(message))
}

pub fn routes(mc: ModelController) -> Router {
    Router::new()
    .route("/messages", post(create_message).get(list_messages))
    .route("/messages/:id", delete(delete_message))
    .route("/messages/:id", put(update_message))
    .with_state(mc)
}