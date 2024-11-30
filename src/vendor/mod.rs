mod rewe;
pub use rewe::{Rewe, ReweConfig};

use super::shopping_list::{Ingredient, IngredientStatus};
use crate::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Vendor {
    Rewe { config: ReweConfig },
}

impl fmt::Display for Vendor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Vendor::Rewe { .. } => write!(f, "rewe"),
        }
    }
}

#[cfg(feature = "ssr")]
impl Vendor {
    pub async fn find_items(&self, ingredient: &mut Ingredient) -> Result<(), String> {
        match self {
            Vendor::Rewe { config } => {
                let rewe = Rewe::new(config.clone());
                rewe.find_items(ingredient).await
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, Hash)]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub quantity: Option<String>,
    pub price_cent: Option<usize>,
    pub url: Option<String>,
    pub image_url: Option<String>,
}

impl Item {
    pub fn id(&self) -> Uuid {
        self.id
    }

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

    pub fn url(&self) -> Option<String> {
        self.url.clone()
    }

    pub fn image_url(&self) -> Option<String> {
        self.image_url.clone()
    }
}
