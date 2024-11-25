use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ProductSearchResult {
    #[serde(rename = "_embedded")]
    pub products: Products,
}

#[derive(Debug, Deserialize)]
pub struct Products {
    pub products: Vec<ProductIn>,
}

#[derive(Debug, Deserialize)]
pub struct ProductIn {
    pub id: String,
    #[serde(rename = "productName")]
    pub name: String,
    pub media: Media,
    #[serde(rename = "_embedded")]
    pub meta: Meta,
}

#[derive(Debug, Deserialize)]
pub struct Media {
    pub images: Vec<Image>,
}

#[derive(Debug, Deserialize)]
pub struct Image {
    #[serde(rename = "_links")]
    pub links: Links,
}

#[derive(Debug, Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub link: SelfLink,
}

#[derive(Debug, Deserialize)]
pub struct SelfLink {
    pub href: String,
}

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub articles: Vec<Article>,
}

#[derive(Debug, Deserialize)]
pub struct Article {
    // pub id: String,
    #[serde(rename = "_embedded")]
    pub article: ArticleData,
}

#[derive(Debug, Deserialize)]
pub struct ArticleData {
    pub listing: Listing,
}

#[derive(Debug, Deserialize)]
pub struct Listing {
    pub pricing: Pricing,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Pricing {
    #[serde(rename = "currentRetailPrice")]
    pub current_retail_price: usize,
    // #[serde(rename = "basePrice")]
    // pub base_price: Option<usize>,
    pub grammage: String,
}
