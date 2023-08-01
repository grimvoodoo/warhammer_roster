mod models;

use models::data_import;
// use std::{fs::File, io::Write};

#[tokio::main]
async fn main() {
    data_import::import().await;
    // let diagram = diagrams::draw_diagrams().await;
    // let mut file = File::create("diagram.dot").expect("Failed to create file");
    // file.write_all(diagram.as_bytes())
    //     .expect("Failed to write to file");
}
