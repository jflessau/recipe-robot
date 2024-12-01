use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: RecordId,
    pub password_hash: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Relation {
    pub r#in: Thing,
    pub out: Thing,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Recipe {
    pub text: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IngredientDb {
    name: String,
    probably_at_home: bool,
}

impl From<Ingredient> for IngredientDb {
    fn from(ingredient: Ingredient) -> Self {
        Self {
            name: ingredient.name,
            probably_at_home: ingredient.probably_at_home,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Requires {
    pub r#in: Thing,
    pub out: Thing,
    pub quantity: i64,
    pub unit: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Seeks {
    pub r#in: Thing,
    pub out: Thing,
    pub quantity: i64,
    pub unit: String,
    pub vendor: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemDb {
    pub name: String,
    pub vendor: String,
    pub price_cent: Option<i64>,
    pub grammage: Option<String>,
    pub url: Option<String>,
    pub image_url: Option<String>,
}

impl From<(Item, &Vendor)> for ItemDb {
    fn from((item, vendor): (Item, &Vendor)) -> Self {
        Self {
            name: item.name,
            vendor: vendor.name(),
            price_cent: item.price_cent,
            grammage: item.grammage,
            url: item.url,
            image_url: item.image_url,
        }
    }
}
