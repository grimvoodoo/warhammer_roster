use bson::{doc, Bson};
use dotenv::dotenv;
use std::{collections::HashMap, fs};

use crate::models::mongodb::{mongodb, Army};

pub async fn import() {
    dotenv().ok();
    // Read the files to memory
    let tyranids = fs::read_to_string("data/tyranids.json").expect("Unable to read file");
    // let custodes = fs::read_to_string("data/custodes.json").expect("Unable to read file");
    // Deserialise the data so we can work with it
    let tyranids_roster: Army =
        serde_json::from_str(&tyranids).expect("failed to convert to string");
    // let custodes_roster: Army =
    //     serde_json::from_str(&custodes).expect("failed to convert to string");

    // Connect to mongodb
    let db = mongodb().await;

    // Write the tyranids to the DB if its empty
    for unit in &tyranids_roster {
        let army = "tyranids";
        let collection = db.collection(army);
        let filter = doc! {"name": &unit.name};
        let result = collection.find_one(filter, None).await;
        match result {
            Ok(Some(_)) => {}
            Ok(None) => {
                println!("importing {} for tyranids from json:", &unit.name);

                let mut models_doc = bson::Document::new();
                for (key, value) in &unit.models {
                    models_doc.insert(key, value);
                }
                let stats_bson = bson::to_bson(&unit.stats)
                    .unwrap()
                    .as_document()
                    .unwrap()
                    .clone();
                let weapons_bson = unit
                    .clone()
                    .weapons
                    .unwrap_or_default()
                    .iter()
                    .map(|model| bson::to_bson(model).unwrap().as_document().unwrap().clone())
                    .collect::<Vec<_>>();
                let abilities_bson: HashMap<String, Bson> = unit
                    .abilities
                    .iter()
                    .map(|(key, value)| {
                        let bson_value = bson::to_bson(value).unwrap();
                        (key.clone(), bson_value)
                    })
                    .collect();
                let mut abilities_doc = bson::Document::new();
                for (key, value) in abilities_bson {
                    abilities_doc.insert(key, value);
                }
                let equipment_bson = match &unit.equipment {
                    Some(equipment) => bson::to_bson(equipment).unwrap(),
                    None => bson::Bson::Array(Vec::new()), // If equipment is None, use an empty BSON array
                };

                let doc = doc! {
                    "name": &unit.name,
                    "points": &unit.points,
                    "stats": stats_bson,
                    "weapons": weapons_bson,
                    "abilities": abilities_doc,
                    "tags": &unit.tags,
                    "models": models_doc,
                    "equipment": equipment_bson
                };
                collection
                    .insert_one(doc, None)
                    .await
                    .expect("Failed to add group");
            }
            Err(_) => todo!(),
        };
    }

    // // Write the custodes to the DB if its empty
    // for unit in &custodes_roster {
    //     let army = "custodes";
    //     let collection = db.collection(army);
    //     if collection
    //         .count_documents(None, None)
    //         .await
    //         .expect("Cannot access database")
    //         == 0
    //     {
    //         println!("importing tyranids from json:");
    //         let models_bson = unit
    //             .models
    //             .iter()
    //             .map(|model| bson::to_bson(model).unwrap().as_document().unwrap().clone())
    //             .collect::<Vec<_>>();

    //         let doc = doc! {
    //             "name": &unit.name,
    //             "points": &unit.points,
    //             "models": models_bson,
    //             "quantity": &unit.quantity,
    //             "tags": &unit.tags
    //         };
    //         collection
    //             .insert_one(doc, None)
    //             .await
    //             .expect("Failed to add group");
    //     } else {
    //         println!("Data already present for {}, not importing", army)
    //     }
    // }
}
