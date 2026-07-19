use crate::database::db_init::get_collection;
use crate::models::client::Client;
use mongodb::{Collection, Cursor, bson::{self, DateTime, oid::ObjectId}, error::Error, options::FindOptions, results::InsertOneResult};
use serde::{Serialize, de::DeserializeOwned};
use tracing::{info,error};



#[derive(Clone, Debug)]
pub struct ClientRepo{
    collections: Collection<Client>,
}

impl ClientRepo{
        pub async fn new() -> Self{
            let col = get_collection("clients").await.expect("faild to get collections");
            Self { collections: col, }
        }
        
        pub async fn create(&self,mut client: Client) -> Result<InsertOneResult,Error>{
            client.created_at = DateTime::now();

            let result = match self.collections.insert_one(&client).await{
                Ok(r) =>{
                    info!("successfully created client");
                    r
                }

                Err(err) =>{
                    error!("failed to create client! {}",err);
                    return Err(err);
                }
            };

            client.id = Some(result.inserted_id.as_object_id().expect("Inserted ID is not an ObjectId"),);
            Ok(result)
        }

        pub async fn find_by_id(&self,id: ObjectId) -> Result<Option<Client>,Error>{
            let filter = bson::doc!{
                "_id": id,
            };
            
            let client =  match self.collections.find_one(filter).await{
                Ok(cl) => {
                    cl
                }
                Err(err) =>{
                    error!("client not found ");
                    return Err(err);
                }
            };

            Ok(client)
        }

        pub async fn find_by_deviceid(&self,device_id: ObjectId) -> Result<Option<Client>,Error>{
            let filter = bson::doc! {
                "device_id": device_id,
            };

            let client = match self.collections.find_one(filter).await{
                Ok(cl) => {
                    cl
                }
                Err(err) =>{
                    error!("client not found");
                    return Err(err);
                }

            };

            Ok(client)
        }

        pub async fn find_all_clients(&self,client_id: ObjectId,limit: i64,skip: u64) -> Result<Cursor<Client>,Error>{
            let filter = bson::doc! {"_id": client_id};


            let opt = FindOptions::builder().limit(limit).skip(skip).sort(bson::doc!{"occurred_at": -1}).build();


            let clients = match self.collections.find(filter).with_options(opt).await{
                Ok(occurred_events) => {
                    occurred_events
                }
                Err(err) =>{
                    return Err(err);
                }
            };

            Ok(clients)

        }


}