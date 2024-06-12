#![allow(unused)]

use axum::*;
use axum::routing::*;
use tower_http::services::ServeDir;
pub use self::error::{Error, Result};

mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()>{
    let mc = model::ModelController::new().await?;
// Lets gather all routes in one place
    let routes_all = Router::new()
    .nest("/api", web::routes_message::routes(mc.clone()))
    .fallback_service(routes_static());
    
// Start server
    let app = routes_all;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
    .await.unwrap();
    println!("->> LISTENING ON: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();


// Fallback service, to serve static files
    fn routes_static() -> Router {
        Router::new().nest_service("/",
        get_service(ServeDir::new("./")))
    }
    Ok(())
}
