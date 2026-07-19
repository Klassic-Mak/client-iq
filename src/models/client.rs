use std::{collections::HashMap};

use serde::{Serialize,Deserialize};
use mongodb::{bson::{oid::ObjectId,DateTime}};



#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Client{
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub client_name: String,
    pub device_id: ObjectId,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    pub created_at: DateTime ,

}


#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct ClientRequest{
    pub client_name: String,
    pub device_id: ObjectId,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}



#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct ClientResponse{
    pub client: Client
}


impl Client{
    pub fn to_response(&self) -> ClientResponse{
        let client = Client{
            id: self.id.clone(),
            client_name :  self.client_name.clone(),
            device_id: self.device_id.clone(),
            metadata: self.metadata.clone(),
            created_at: self.created_at.clone(),
        };

        ClientResponse { 
            client: client,
         }
    }
}