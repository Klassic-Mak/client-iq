mod database;
mod config; 
mod models;
mod repositories;
mod services;
mod handlers;
use std::sync::Arc;

use crate::config::config::Config; 
use crate::database::db_init;
use crate::handlers::client_handler::ClientHandler;
use crate::repositories::{client_repo::ClientRepo,device_repo::DeviceRepo,event_repo::EventRepo};
use crate::services::client_service;
use crate::services::{client_service::ClientService,device_service,event_service};
use crate::handlers::websocket_handler::{self, AppState};
use crate::handlers::{websocket_handler::websocket_handler,client_handler::{create_client_handler,
get_all_clients_handler,get_by_device_id_handler,get_by_id_handler}};
use axum::routing::post;
use tracing::info;
use tracing_subscriber;
use tokio;
use axum::{Router, routing::{any, get}};



#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app_config = Config::load_config().expect("failed to load config");

    //Initialized the database 
    let _ = db_init::database_connection().await;

    //Initialized repositories
    let client_repo = ClientRepo::new().await;
    let device_repo = DeviceRepo::new().await;
    let event_repo = EventRepo::new().await;


    //Initialized services
    let client_service = Arc::new(ClientService{
        client_repo,
        device_repo
    });

    // Create the AppState
    let app_state = AppState {
        client_service: client_service.clone(),
    };

    

    // Build the Router
    let port = &app_config.port;
    let addr = format!("0.0.0.0:{}", port);
    let app = Router::new()
                            .route("/ws", get(test_db_handler))
                            .route("/ws/client/{client_id}", any(websocket_handler).with_state(app_state.clone()))
                            .route("/api/v1/create-client", post(create_client_handler).with_state(app_state.clone()))
                            .route("/api/v1/clients", get(get_all_clients_handler).with_state(app_state.clone()))
                            .route("/api/v1/client/{id}", get(get_by_id_handler).with_state(app_state.clone()));

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
    
    info!("Server successfully running on {}", addr);
    axum::serve(listener, app).await.unwrap();

}


async fn test_db_handler() -> String {
    // This uses your global DB and your get_collection function
    info!("Hello, World");
    "Hello World".to_string()
}