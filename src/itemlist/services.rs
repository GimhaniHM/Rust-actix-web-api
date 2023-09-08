use actix_web :: {get, post, put, web, Responder, HttpResponse};
use crate :: {AppState, GroceryItem, InvoiceEntry};
use super::models::{CreateEntryData, CreateInvoiceEntryData, UpdateEntryData};

// #[get("/")]
// async fn initial() -> String {
//     "This is a health check".to_string()
// }

////----------------------  START - Item stock controllers ----------------------------- ////

//route for adding items to hashmap
#[post("/itemlist/stock_entries")]
async fn create_entry(data: web::Data<AppState>, param_obj: web::Json<CreateEntryData>) -> impl Responder {
    let mut itemlist_collection = data.item_collection.lock().unwrap();
    itemlist_collection.insert(param_obj.code.clone(), GroceryItem {
        name: param_obj.name.clone(),
        price: param_obj.price,
        qty: param_obj.qty
    });

    HttpResponse::Ok().json(&*itemlist_collection)
}

//route for getting items from hashmap(All items)
#[get("/itemlist/stock_entries")]
async fn get_entries(data: web::Data<AppState>) -> impl Responder {
    // Lock the Mutex to access the HashMap
    let item_collection = data.item_collection.lock().unwrap();

    HttpResponse::Ok().json(&*item_collection)
}

//route for updating items from hashmap by using item code
#[put("/itemlist/stock_entries/{code}")]
async fn update_entry(data: web::Data<AppState>,path: web::Path<String>,param_obj: web::Json<UpdateEntryData>) -> impl Responder {
    
    let code = path.into_inner();
    let mut item_collection = data.item_collection.lock().unwrap();
    
    if let Some(item) = item_collection.get_mut(&code) {
        item.qty = item.qty - param_obj.qty;
        HttpResponse::Ok().json(&*item_collection)
    } else  {
        HttpResponse::BadRequest().body("Item not found in itemlist_collection")
    } 

}


////----------------------  END - Item stock controllers ----------------------------- ////



////----------------------  START - Invoice Item stock controllers ----------------------------- ////

//route for adding invoice items to hashmap
#[post("/itemlist/invoice_entries")]
async fn create_invoice_entry(data: web::Data<AppState>, param_obj: web::Json<CreateInvoiceEntryData>) -> impl Responder {
    let itemlist_collection = data.item_collection.lock().unwrap();
    let mut invoicelist_collection = data.invoice_collection.lock().unwrap();

    match itemlist_collection.get(&param_obj.code) {
        Some(data) => {
            if param_obj.qty <= data.qty {
                invoicelist_collection.insert(param_obj.code.clone(),InvoiceEntry{
                    name: data.name.clone(),
                    qty: param_obj.qty,
                    price: data.price,
                    total: param_obj.qty as f32 * data.price,
                    }
                );

            HttpResponse::Ok().json(&*invoicelist_collection)
                
            } else {
                HttpResponse::BadRequest().body("Not enough quantity available")
            } 
        },
        None => {
            HttpResponse::BadRequest().body("Item not found in itemlist_collection")
        }

    }

    
}

//route for updating inivoice items quantity from hashmap by using item code if already hashmap contains the paticular data
#[put("/itemlist/invoice_entries/{code}")]
async fn update_invoice_entry(data: web::Data<AppState>,path: web::Path<String>,param_obj: web::Json<UpdateEntryData>) -> impl Responder {
    
    let code = path.into_inner();
    let mut invoicelist_collection = data.invoice_collection.lock().unwrap();
    
    if let Some(data) = invoicelist_collection.get_mut(&code){
        data.qty += param_obj.qty;
        data.total += param_obj.qty as f32 * data.price;
        HttpResponse::Ok().json(&*invoicelist_collection)
    } else {
        HttpResponse::BadRequest().body("Item not found in invoice_collection")
    }
    

}


//Controller for viewing the invoicelist items
#[get("/itemlist/invoice_entries/")]
async fn get_invoice_entries(data: web::Data<AppState>) -> impl Responder {
    let invoicelist_collection = data.invoice_collection.lock().unwrap();

    HttpResponse::Ok().json(&*invoicelist_collection)
}


//Controller for add the subtotal of invoicelist
#[get("/itemlist/invoice_entries/sub_total")]
async fn get_invoice_subtotal(data: web::Data<AppState>) -> impl Responder {
    let invoicelist_collection = data.invoice_collection.lock().unwrap();

    if invoicelist_collection.is_empty() {
        HttpResponse::BadRequest().body("Invoice list is empty")
    } else {
        // Calculate the subtotal
        let sub_total: f32 = invoicelist_collection.values().map(|item| item.total).sum();

        // Lock the sub_total field and update it
        let mut state_sub_total = data.sub_total.lock().unwrap();
        *state_sub_total = sub_total;

        HttpResponse::Ok().json(sub_total)
    }
}


////----------------------  END - Invoice Item stock controllers ----------------------------- ////

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_entry)
    .service(get_entries)
    .service(create_invoice_entry)
    .service(update_entry)
    .service(update_invoice_entry)
    .service(get_invoice_entries)
    .service(get_invoice_subtotal);

    
}