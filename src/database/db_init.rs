use crate::config::config::Config;
use std::error::Error;
use once_cell::sync::OnceCell;
use serde::de::DeserializeOwned;
use tracing::info;
use mongodb::{Client, Collection, Database, options::{ClientOptions, ServerApi, ServerApiVersion}};

pub struct Dbase{
    pub client: Client,
    pub db: Database
}

static DB: OnceCell<Dbase> =  OnceCell::new();

pub async fn database_connection() -> Result<Client, Box<dyn Error>> {
  
    let app_config = Config::load_config()?;
    

    let mut client_options = ClientOptions::parse(&app_config.databse_url).await?;

    // Set the server_api field of the client_options object to set the version of the Stable API on the client
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;

    

    // Ping the server to see if you can connect to the cluster
    let dbase = client.database(&app_config.db_name);
    info!("Pinged your deployment. You successfully connected to MongoDB!");

    DB.set(Dbase { client: client.clone(), db: dbase.clone() })
        .map_err(|_| "Database already initialized")?;


    Ok(client)
}

pub async fn get_collection<T>(name: &str)  -> Result<Collection<T>,Box<dyn Error>> 
  where T:DeserializeOwned + Send + Sync + Unpin,
  {
    let db = DB.get().expect("Database not initialized! Call init_db first.");
    Ok(db.db.collection(name)) 
}