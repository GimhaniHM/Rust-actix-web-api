use serde::Deserialize;

//// -------------------------------- START ------------------------------////
#[derive(Deserialize, Clone)]
pub struct CreateEntryData {
    pub code: String,
    pub name: String,
    pub price: f32,
    pub qty: i32
}

#[derive(Deserialize, Clone)]
pub struct UpdateEntryData {
    pub qty: i32
}

//// --------------------------------END ------------------------------////


//// --------------------------------START ------------------------------////

#[derive(Deserialize, Clone)]
pub struct CreateInvoiceEntryData {
    pub code: String,
    pub qty: i32
}

#[derive(Deserialize, Clone)]
pub struct UpdateInvoiceEntryData {
    pub qty: i32
}


//// --------------------------------END ------------------------------////




