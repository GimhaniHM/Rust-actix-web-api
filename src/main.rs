use actix_web::{get, web, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::{sync::{Mutex}, collections::HashMap};

mod itemlist;
use itemlist::services;

//use to store application data
struct AppState {
    item_collection: Mutex<HashMap<String, GroceryItem>>,
    invoice_collection: Mutex<HashMap<String, InvoiceEntry>>,
    sub_total: Mutex<f32>
}

//use to store invoice items
#[derive(Serialize, Deserialize, Clone)]
struct InvoiceEntry {
    name: String,
    qty: i32,
    price: f32,
    total: f32
}

#[derive(Serialize, Deserialize, Clone)]
struct GroceryItem {
    name: String,
    price: f32,
    qty: i32
}

//define route & route handler function 
#[get("/")]
async fn index() -> String {
    "This is a health check =====".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let data = web::Data::new({
        AppState {
            item_collection: Mutex::new(HashMap::new()),
            invoice_collection: Mutex::new(HashMap::new()),
            sub_total: Mutex::new(0.0)
        }
    });

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        App::new().app_data(data.clone()).service(index).configure(services::config)
    })
    .bind(("127.0.0.1", 5000))?
        .run()
        .await
    
}
