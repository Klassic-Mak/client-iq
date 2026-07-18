mod database;
mod config; 
use crate::config::config::Config; 
use crate::database::db_init;
use tracing::info;
use tracing_subscriber;
use tokio;
use axum::{Router, routing::get};



#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app_config = Config::load_config().expect("failed to load config");
    let _ = db_init::database_connection().await;

    let port = &app_config.port;

    let addr = format!("0.0.0.0:{}", port);

    let app = Router::new().route("/ws", get(test_db_handler().await));

    let listener = match tokio::net::TcpListener::bind(&addr).await{
        Ok(l) =>{
            info!("Server successfully running on {}", addr);
            l
        }
        Err(e)=>{
            info!("Failed to bind to {}: {}", addr, e);
            return;
        }
    };
    
    axum::serve(listener, app).await.unwrap();

}


async fn test_db_handler() -> String {
    // This uses your global DB and your get_collection function
    info!("Hello, World");
    "Hello World".to_string()
}