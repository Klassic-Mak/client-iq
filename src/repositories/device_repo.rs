use crate::models::device;
use crate::{database::db_init::get_collection, models::device::DeviceProfile};
use crate::models::client::Client;
use mongodb::Cursor;
use mongodb::options::FindOptions;
use mongodb::{Collection, bson::{DateTime, oid::ObjectId}, error::Error, results::InsertOneResult,bson};
use serde::{Serialize, de::DeserializeOwned};
use tracing::{info,error};



#[derive(Clone, Debug)]
pub struct DeviceRepo{
    collections: Collection<DeviceProfile>,
}

impl DeviceRepo{
        pub async fn new() -> Self{
            let col = get_collection("device").await.expect("faild to get collections");
            Self { collections: col, }
        }
        
        pub async fn create(&self,mut device: DeviceProfile) -> Result<InsertOneResult,Error>{
            device.last_seen_at = DateTime::now();

            let result = match self.collections.insert_one(&device).await{
                Ok(r) =>{
                    info!("successfully created device");
                    r
                }

                Err(err) =>{
                    error!("failed to create device! {}",err);
                    return Err(err);
                }
            };

            device.id = Some(result.inserted_id.as_object_id().expect("Inserted ID is not an ObjectId"),);
            Ok(result)
        }

        pub async fn find_by_id(&self,id: ObjectId) -> Result<Option<DeviceProfile>,Error>{
            let filter = bson::doc!{
                "_id": id,
            };
            
            let device =  match self.collections.find_one(filter).await{
                Ok(cl) => {
                    cl
                }
                Err(err) =>{
                    error!("client not found ");
                    return Err(err);
                }
            };

            Ok(device)
        }

        pub async fn find_by_clientid(&self,client_id: ObjectId) -> Result<Option<DeviceProfile>,Error>{
            let filter = bson::doc! {
                "client_id": client_id,
            };

            let device = match self.collections.find_one(filter).await{
                Ok(cl) => {
                    cl
                }
                Err(err) =>{
                    error!("device not found");
                    return Err(err);
                }

            };

            Ok(device)
        }

        pub async fn find_all_device(&self,device_id: ObjectId,limit: i64,skip: u64) -> Result<Cursor<DeviceProfile>,Error>{
            let filter = bson::doc!{"_id": device_id};

            let opt = FindOptions::builder().limit(limit).skip(skip).sort(bson::doc!{"last_seen_at": -1}).build();

            let devices = match self.collections.find(filter).await{
                Ok(cursor) => {
                    cursor
                }
                Err(err) => {
                    error!("device not found");
                    return Err(err);
                }
            };

            Ok(devices)


        }

        
}