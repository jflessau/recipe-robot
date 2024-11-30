use crate::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Vendor {
    Rewe { zip_code: String },
}

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
