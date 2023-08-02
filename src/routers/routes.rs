use crate::models::{
    db_read::{read_army, read_list},
    db_write::write_list,
    structs::Marker,
};
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/units")]
pub async fn get_units() -> impl Responder {
    let units = read_army("tyranids").await; // Call your function to get units
    HttpResponse::Ok().json(units) // Respond with JSON
}

#[get("/list")]
pub async fn get_list() -> impl Responder {
    let units: Vec<crate::models::structs::Marker> = read_list("grim").await; // Call your function to get units
    HttpResponse::Ok().json(units) // Respond with JSON
}

#[post("/units")]
pub async fn create_unit(unit: web::Json<Marker>) -> impl Responder {
    match write_list("grim", unit).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }; // Call your function to write unit
    HttpResponse::Created().finish()
}