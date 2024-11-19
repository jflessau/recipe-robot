mod rewe;
pub use rewe::{Rewe, ReweConfig};

use super::*;

use serde::{Deserialize, Serialize};

// const THEMES: [&str; 14] = [
//     "organic",
//     "cheap",
//     "regional",
//     "vegan",
//     "vegetarian",
//     "gluten-free",
//     "lactose-free",
//     "sugar-free",
//     "low-carb",
//     "low-fat",
//     "low-salt",
//     "low-calorie",
//     "high-protein",
//     "high-fiber",
// ];

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum VendorSelect {
    Rewe { config: ReweConfig },
}

pub trait Vendor
where
    Self: Sized,
{
    type Config;

    async fn new(config: Self::Config) -> Result<Self>;

    fn name(&self) -> String;

    async fn search_for_items(&self, ingredient: Ingredient) -> Result<Vec<Item>>;

    // automatically implemented

    async fn match_item(
        &self,
        ingredient: Ingredient,
        themes: &Vec<String>,
        ai: &Ai,
    ) -> Result<Ingredient> {
        ai.match_item(ingredient, themes).await
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Item {
    name: String,
    quantity: Option<String>,
    price_cent: Option<usize>,
    url: Option<String>,
    image_url: Option<String>,
}

impl Item {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn quantity(&self) -> String {
        self.quantity.clone().unwrap_or("?".to_string())
    }

    pub fn price(&self) -> f32 {
        self.price_cent.map(|p| p as f32 / 100.0).unwrap_or(0.0)
    }

    pub fn price_string(&self) -> String {
        format!("{:.2}", self.price())
    }

    pub fn price_total(&self, pieces: usize) -> f32 {
        self.price_cent
            .map(|p| p as f32 / 100.0 * pieces as f32)
            .unwrap_or(0.0)
    }

    pub fn price_total_string(&self, pieces: usize) -> String {
        format!("{:.2}", self.price_total(pieces))
    }
}
