use bson::{doc, Bson, Document};
use dotenv::dotenv;
use futures::StreamExt;
use mongodb::Collection;

use crate::models::mongodb::mongodb;

use super::structs::{Marker, Unit};

pub async fn read_army(army: &str) -> Vec<Unit> {
    dotenv().ok();
    let db = mongodb("roster").await;
    let collection: Collection<Document> = db.collection(army);
    let filter = doc! {};
    let mut cursor = collection.find(filter, None).await.unwrap();
    let mut results = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let unit: Unit = bson::from_bson(Bson::Document(document)).unwrap();
                results.push(unit);
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    results
}

pub async fn read_list(user: &str) -> Vec<Marker> {
    dotenv().ok();
    let db = mongodb("users").await;
    let collection: Collection<Document> = db.collection(user);
    let filter = doc! {};
    let mut cursor = collection.find(filter, None).await.unwrap();
    let mut results = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let unit: Marker = bson::from_bson(Bson::Document(document)).unwrap();
                results.push(unit);
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    results
}
