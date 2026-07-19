use axum::{
    Json, extract::{Path, Query, State}, http::StatusCode, response::IntoResponse,
};
use mongodb::{bson::oid::ObjectId, error::Error};
use serde::Deserialize;
use tracing::error;

use crate::{
    handlers::websocket_handler::AppState, models::client::{ClientRequest, ClientResponse}, services::client_service::ClientService,
};

#[derive(Deserialize)]
pub struct Pagination {
    page: u64,
    limit: i64,
}

#[derive(Clone)]
pub struct ClientHandler {
    pub client_service: ClientService,
}

impl ClientHandler {
    pub fn new(client_service: ClientService) -> Self {
        Self { client_service }
    }
}

pub async fn create_client_handler(
    State(h): State<AppState>,
    Json(request): Json<ClientRequest>,
) -> impl IntoResponse {
    match h.client_service.create_client(request).await {
        Ok(cl) => (StatusCode::CREATED, Json(cl)).into_response(),
        Err(err) => {
            error!("failed to create client");
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}


pub async fn get_by_id_handler(State(h): State<AppState>,Json(id): Json<ObjectId>,) -> impl IntoResponse {
    match h.client_service.get_by_id(id).await {
        Ok(cl) => (StatusCode::OK, Json(cl)).into_response(),
        Err(err) => {
            error!("Error fetching client {}: {}", id, err);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}

pub async fn get_by_device_id_handler( State(h): State<AppState>,Json(device_id): Json<ObjectId>,) -> impl IntoResponse {
    match h.client_service.get_by_deviceid(device_id).await {
        Ok(cl) => (StatusCode::OK, Json(cl)).into_response(),
        Err(err) => {
            error!("client not found {}", device_id);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}

pub async fn get_all_clients_handler( State(h): State<AppState>,Path(client_id): Path<String>, Query(params): Query<Pagination>,) -> impl IntoResponse {
    let client_id = ObjectId::parse_str(client_id).unwrap(); 
    
    match h.client_service.get_all_clients(client_id, params.limit, params.page).await {
        Ok(clients) => {
            let response: Vec<ClientResponse> = clients
                .into_iter()
                .map(|c| ClientResponse { client: c })
                .collect();
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => {
            error!("Error fetching clients: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to retrieve clients").into_response()
        }
    }
}