use std::collections::HashMap;
use mongodb::bson::{DateTime, oid::ObjectId};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize,Clone)]
#[serde(rename_all = "snake_case")]
pub enum DeviceStatus {
    Active,
    Offline,
    Suspended,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct DeviceProfile {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    
    pub device_id: String,
    
    pub status: DeviceStatus,
    
    pub total_event_count: u64,
    pub last_seen_at: DateTime,
    
    pub device_config: HashMap<String, serde_json::Value>,
}