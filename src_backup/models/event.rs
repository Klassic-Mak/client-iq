use std::collections::HashMap;

use mongodb::bson::{DateTime, oid::ObjectId};
use serde::{Serialize,Deserialize};

#[derive(Debug,Deserialize,Serialize,Clone)]
#[serde(rename_all = "snake_case")] 
pub enum EventType {
    ErrorReport,
    SessionStart,
    InteractionLog,
}



#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct EventData{
    #[serde(rename="_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub client_id: ObjectId,
    pub device_id: ObjectId,
    pub event_type: EventType,
    pub event_metadata: HashMap<String,serde_json::Value>,
    pub occurred_at: DateTime,
}



