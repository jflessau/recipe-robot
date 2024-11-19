use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Product {
    pub name: String,
    pub price: usize,
    pub grammage: String,
    pub url: String,
    pub image: String,
}

// #[derive(Debug, Deserialize)]
// pub struct Paging {
//     objects_per_page: Option<u32>,
//     current_page: Option<u32>,
//     page_count: Option<u32>,
//     object_count: Option<u32>,
// }

#[derive(Debug, Deserialize)]
pub struct ProductListing {
    pub products: Vec<Product>,
    // paging: Paging,
}
