//This is a handler that uses the create a create_groceryitem

use crate::{models::stock_model::GroceryItem, repository::mongo_repo::MongoRepo};

use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};

#[post("/groceryitem")]
pub async fn create_groceryitem(db: Data<MongoRepo>, new_groceryitem: Json<GroceryItem>) -> HttpResponse {
    let data = GroceryItem {
        name: new_groceryitem.name.to_owned(),
        price: new_groceryitem.price.to_owned(),
        qty: new_groceryitem.qty.to_owned(),
    };
    let groceryitem_detail = db.create_groceryitem(data).await;
    match groceryitem_detail {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}