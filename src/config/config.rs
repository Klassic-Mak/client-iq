use std::{ error::Error,env};
use dotenvy::dotenv;
use tracing::{info,error};

pub struct Config{
    pub db_name: String,
    pub databse_url: String,
    pub port: String,
}


impl Config {

    pub fn load_config() -> Result<Config, Box<dyn Error>> {
        // Handle dotenv initialization
        if let Err(_) = dotenv() {
            error!("env file does not exist!");
        }

        let db_name = env::var("DB_NAME") .map_err(|_| {
                error!("DB_NAME must be set");
                "DB_NAME missing"
            })?;

        let databse_url = env::var("DATABASE_URL").map_err(|_| {
                error!("DATABASE_URL must be set");
                "DATABASE_URL missing"
            })?;
        let port = env::var("PORT").map_err(|_| {
            error!("PORT must be set");
            "PORT missing"
        })?;

        info!("Configuration loaded successfully.");
        
        Ok(Config {
            db_name,
            databse_url,
            port,
        })
    }
}