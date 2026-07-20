use crate::repositories::client_repo::ClientRepo;
use crate::repositories::device_repo::DeviceRepo;
use crate::models::client::{ClientRequest,ClientResponse,Client};
use mongodb::bson::DateTime;
use mongodb::bson::oid::ObjectId;
use mongodb::error::Error;
use futures::stream::TryStreamExt;



#[derive(Debug,Clone)]
pub struct ClientService{
    pub client_repo: ClientRepo,
    pub device_repo: DeviceRepo,
}


impl ClientService{
    pub async fn new(&self) -> Self{
        Self{
            client_repo: self.client_repo.clone(),
            device_repo: self.device_repo.clone()
        }
    }

    pub async fn create_client(&self,client_req: ClientRequest) -> Result<ClientResponse,Error>{
        // let device = self.device_repo.find_by_id(client_req.device_id).await?.ok_or_else(|| Error::from(std::io::Error::new(
        //     std::io::ErrorKind::NotFound, "Device not found" )))?;

        
        let  new_client = Client{
            id: Some(ObjectId::new()),
            device_id: client_req.device_id,  //device.id.unwrap_or(client_req.device_id),
            client_name: client_req.client_name,
            metadata: client_req.metadata,
            created_at: DateTime::now()
        };

        let client = match self.client_repo.create(new_client.clone()).await{
            Ok(cl) => cl,
            Err(err) =>{
                return Err(err);
            }
        };

        let client_reponse = new_client.to_response();

        Ok(client_reponse)
    }

    pub async fn get_by_id(&self,id: ObjectId) -> Result<ClientResponse,Error>{
        let client = self.client_repo.find_by_id(id).await?.ok_or_else(|| Error::from(std::io::Error::new(
            std::io::ErrorKind::NotFound, "Client not found"
        )))?;

        Ok(client.to_response())
    }

    pub async fn get_by_deviceid(&self,device_id: ObjectId) -> Result<ClientResponse,Error>{
        let client = self.client_repo.find_by_deviceid(device_id).await?.ok_or_else(|| Error::from(std::io::Error::new(
            std::io::ErrorKind::NotFound, "Client not found"
        )))?;

        Ok(client.to_response())
    }

    pub async fn get_all_clients(&self, client_id: ObjectId, limit: i64, page: u64) -> Result<Vec<Client>, Error> {
        let safe_limit = if limit <= 0 || limit > 100 { 20 } else { limit };
        let safe_page = if page < 1 { 1 } else { page };
        
        let skip = ((safe_page - 1) as i64) * safe_limit;
    
        let mut cursor = self.client_repo.find_all_clients(client_id, safe_limit, skip as u64).await?;
    
        let clients: Vec<Client> = cursor.try_collect().await?;
    
        Ok(clients)
    }
}