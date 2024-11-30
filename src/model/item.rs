use crate::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, Hash)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub quantity: Option<String>,
    pub price_cent: Option<usize>,
    pub url: Option<String>,
    pub image_url: Option<String>,
}

impl Item {
    pub fn price_total(&self, pieces: usize) -> f32 {
        self.price_cent
            .map(|p| p as f32 / 100.0 * pieces as f32)
            .unwrap_or(0.0)
    }

    pub fn price_total_string(&self, pieces: usize) -> String {
        format!("{:.2}", self.price_total(pieces))
    }
}
