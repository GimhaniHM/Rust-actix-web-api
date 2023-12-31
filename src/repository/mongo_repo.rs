use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error},
    results::{ InsertOneResult},
    options::ClientOptions,
    Client, Collection,
};

use crate::models::stock_model::GroceryItem;

pub struct MongoRepo {
    col: Collection<GroceryItem>,
}

impl MongoRepo {
    pub async fn init() -> Result<Self, mongodb::error::Error> {
        // dotenv().ok();  // init env
        // let uri = match env::var("DATABASE_URL") {
        //     Ok(v) => v.to_string(),
        //     Err(_) => format!("Error loading env variable"),
        // };
        let uri = "mongodb+srv://user-name:u@123@billingdb.7966em3.mongodb.net/?retryWrites=true&w=majority";
        let mut client_options = ClientOptions::parse(uri).await?;
        let client = Client::with_options(client_options).unwrap();
        let db = client.database("billingdb");
        let col: Collection<GroceryItem> = db.collection("GroceryItem");
        println!("✅ Database connected successfully");
        Ok(Self {
            col
        })
    }

    pub async fn create_groceryitem(&self, new_groceryitem: GroceryItem) -> Result<InsertOneResult, Error> {
        let new_doc = GroceryItem {
            name: new_groceryitem.name,
            price: new_groceryitem.price,
            qty: new_groceryitem.qty
        };

        let groceryitem = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating grocery item");

            Ok(groceryitem)
    }
}