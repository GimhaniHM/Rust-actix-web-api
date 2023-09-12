
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GroceryItem {
    pub name: String,
    pub price: f32,
    pub qty: i32
}