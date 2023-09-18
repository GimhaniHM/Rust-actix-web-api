use actix_web::{get, web::{self, Data}, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::{sync::{Mutex}, collections::HashMap};


// mod itemlist;
// use itemlist::services;

mod api;
mod models;
mod repository;

use api::stock_api::{create_groceryitem, get_name};


use repository::mongo_repo::MongoRepo;

use dotenv::dotenv;

//use to store application data
// struct AppState {
//     item_collection: Mutex<HashMap<String, GroceryItem>>,
//     invoice_collection: Mutex<HashMap<String, InvoiceEntry>>,
//     sub_total: Mutex<f32>
// }

//use to store invoice items
#[derive(Serialize, Deserialize, Clone)]
struct InvoiceEntry {
    name: String,
    qty: i32,
    price: f32,
    total: f32
}

// #[derive(Serialize, Deserialize, Clone)]
// struct GroceryItem {
//     name: String,
//     price: f32,
//     qty: i32
// }

//define route & route handler function 
#[get("/")]
async fn index() -> String {
    "This is a health check =====".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    // let data = web::Data::new({
    //     AppState {
    //         item_collection: Mutex::new(HashMap::new()),
    //         invoice_collection: Mutex::new(HashMap::new()),
    //         sub_total: Mutex::new(0.0)
    //     }
    // });

    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        App::new().app_data(db_data.clone())
            .service(get_name)
            .service(index)
            .service(create_groceryitem)
    })
    .bind(("127.0.0.1", 8080))?
        .run()
        .await
    
}
