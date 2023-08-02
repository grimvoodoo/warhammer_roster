mod models;
mod routers;

use actix_web::{App, HttpServer};
use routers::routes::{create_unit, get_units};

use crate::{models::db_write::import_from_json, routers::routes::get_list};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    import_from_json().await;
    println!("Server running at 127.0.0.1:8000");
    HttpServer::new(|| {
        App::new()
            .service(get_units)
            .service(create_unit)
            .service(get_list)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
