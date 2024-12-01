mod rewe;

use crate::prelude::*;
use rewe::Rewe;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Vendor {
    Rewe { zip_code: String },
}

impl Vendor {
    pub async fn find_items(&self, ingredient: &mut Ingredient) -> Result<(), Error> {
        match self {
            Vendor::Rewe { zip_code } => {
                let rewe = Rewe::new(zip_code.clone());
                rewe.find_items(ingredient).await
            }
        }
    }

    pub fn name(&self) -> String {
        match self {
            Vendor::Rewe { .. } => "rewe".to_string(),
        }
    }
}
