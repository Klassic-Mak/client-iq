use crate::database::db_init::get_collection;
use crate::models::event::{EventData,EventType};
use mongodb::Cursor;
use mongodb::options::FindOptions;
use mongodb::{Collection, bson::{DateTime, oid::ObjectId}, error::Error, results::InsertOneResult,bson};
use tracing::{info,error};



#[derive(Clone, Debug)]
pub struct EventRepo{
    collections: Collection<EventData>,
}

impl EventRepo{
        pub async fn new() -> Self{
            let col = get_collection("event").await.expect("faild to get collections");
            Self { collections: col, }
        }
        
        pub async fn create(&self,mut event: EventData) -> Result<InsertOneResult,Error>{
            event.occurred_at = DateTime::now();

            let result = match self.collections.insert_one(&event).await{
                Ok(r) =>{
                    info!("successfully created event");
                    r
                }

                Err(err) =>{
                    error!("failed to create event! {}",err);
                    return Err(err);
                }
            };

            event.id = Some(result.inserted_id.as_object_id().expect("Inserted ID is not an ObjectId"),);
            Ok(result)
        }

        pub async fn find_by_id(&self,id: ObjectId) -> Result<Option<EventData>,Error>{
            let filter = bson::doc!{
                "_id": id,
            };
            
            let event =  match self.collections.find_one(filter).await{
                Ok(occurred_event) => {
                    occurred_event
                }
                Err(err) =>{
                    error!("client not found ");
                    return Err(err);
                }
            };

            Ok(event)
        }

        pub async fn find_by_deviceid(&self,device_id: ObjectId) -> Result<Option<EventData>,Error>{
            let filter = bson::doc! {
                "device_id": device_id,
            };

            let event = match self.collections.find_one(filter).await{
                Ok(occurred_event) => {
                    occurred_event
                }
                Err(err) =>{
                    error!("client not found");
                    return Err(err);
                }

            };

            Ok(event)
        }

        pub async fn find_by_clientid(&self,client_id: ObjectId) -> Result<Option<EventData>,Error>{
            let filter = bson::doc! {"client_id": client_id};

            let event = match self.collections.find_one(filter).await{
                Ok(occurred_event) => {
                    occurred_event
                }
                Err(err) =>{
                    error!("event not found");
                    return Err(err)
                }
            };

            Ok(event)
        }

        pub async fn find_all_event(&self,event_id: ObjectId,limit: i64,skip: u64) -> Result<Cursor<EventData>,Error>{
            let filter = bson::doc! {"_id": event_id};


            let opt = FindOptions::builder().limit(limit).skip(skip).sort(bson::doc!{"occurred_at": -1}).build();


            let events = match self.collections.find(filter).with_options(opt).await{
                Ok(occurred_events) => {
                    occurred_events
                }
                Err(err) =>{
                    return Err(err);
                }
            };

            Ok(events)

        }

        
}