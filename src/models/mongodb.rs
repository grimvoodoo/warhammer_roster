use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client, Database};
use std::env;

use super::models::Unit;
pub type Army = Vec<Unit>;

pub async fn mongodb() -> Database {
    dotenv().ok();
    let uri = match env::var("MONGODB_URI") {
        Ok(v) => v,
        Err(_) => format!("Error loading env variable"),
    };
    let client_options = ClientOptions::parse(uri)
        .await
        .expect("Failed to parse connection string");
    let client = Client::with_options(client_options).expect("Failed to connect to db");
    // Set DB, this will be created if it doesn't already exist
    client.database("warhammer")
}
