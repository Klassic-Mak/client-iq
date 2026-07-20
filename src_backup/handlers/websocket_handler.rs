use axum::{Error, Router, extract::{Path, Query, State, ws::{Message, WebSocket, WebSocketUpgrade}}, response::IntoResponse, routing::get}; 
use futures_util::{sink::SinkExt, stream::{StreamExt, SplitSink, SplitStream}};
use mongodb::bson::oid::ObjectId;
use serde::Deserialize;
use crate::models::client::{Client, ClientRequest, ClientResponse};
use crate::services::client_service::ClientService;
use std::sync::Arc;





#[derive(Clone)]
pub struct AppState {
    pub client_service: Arc<ClientService>,
    // pub device_service: Arc<DeviceService>,
    // pub event_service: Arc<EventService>,
}


pub async fn websocket_handler(
    Path(client_id): Path<String>, // Extracts the {client_id} from the path
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    // Now you have client_id from the URL path
    ws.on_upgrade(move |socket| handle_socket(socket, state, client_id))
}


async fn handle_socket(mut socket: WebSocket, state: AppState,client_id: String) {

    let (mut sender, mut receiver) = socket.split();

   let read_task = tokio::spawn(write(sender,state.client_service.clone(),client_id));
    let write_task = tokio::spawn(read(receiver,state.client_service.clone()));

    tokio::select! {
        _ = read_task => (),
        _ = write_task => (),
    }
}

pub async fn read(mut receiver: SplitStream<WebSocket>, service: Arc<ClientService>) {
    while let Some(result) = receiver.next().await {
        match result {
            Ok(Message::Text(text)) => {
                // Safely handle JSON parsing
                if let Ok(client) = serde_json::from_str::<Client>(&text) {
                    println!("Received Client: {:?}", client);
                    // You can now call your service: 
                    let client_request = ClientRequest{
                        device_id: client.device_id.clone(),
                        client_name: client.client_name.clone(),
                        metadata: client.metadata.clone(),
                    };
                    let _ = service.create_client(client_request.into()).await;
                }
            }
            Ok(Message::Close(_)) => break,
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }
}

async fn write(mut sender: SplitSink<WebSocket, Message>,service: Arc<ClientService>,client_id:String ) {
    tokio::spawn(async move{
        let client_id = ObjectId::parse_str(client_id).unwrap();
        let limit = 20;
        let page = 1;

        loop{
            match service.get_all_clients(client_id, limit, page).await {
                Ok(client) => {
                            if let Ok(json_data) =  serde_json::to_string(&client){
                                if sender.send(Message::Text(json_data.into())).await.is_err() { break; }
                            }
                            
                        }
                Err(e) => eprintln!("Failed to fetch client data: {}", e),
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    });
}