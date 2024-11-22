mod ingredient;

use super::{
    ai::Ai,
    vendor::{Rewe, ReweConfig, Vendor, VendorSelect},
};
pub use ingredient::{Ingredient, IngredientStatus};

use crate::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ShoppingList {
    recipe: String,
    ingredients: Vec<Ingredient>,
    themes: Vec<String>,

    vendor: VendorSelect,

    ai: Ai,

    sub_total_without_at_home_items: f32,
    total: f32,
}

impl ShoppingList {
    pub fn new(recipe: String, themes: Vec<String>) -> Self {
        let ai = Ai::new(7000).expect("failed to create ai");
        let vendor = VendorSelect::Rewe {
            config: ReweConfig { zip_code: 10961 },
        };

        Self {
            recipe,
            ingredients: vec![],
            themes,

            vendor,

            ai,

            sub_total_without_at_home_items: 0.0,
            total: 0.0,
        }
    }

    pub async fn find_ingredients(&mut self) -> Result<()> {
        info!("🤖 list ingredients...");
        let ingredients = self
            .ai
            .get_ingredients(&self.recipe)
            .await
            .context("failed to get ingredients")?;

        self.ingredients = ingredients;

        self.total = self
            .ingredients
            .clone()
            .iter()
            .flat_map(|i| i.price_total())
            .sum::<f32>();

        self.sub_total_without_at_home_items = self
            .ingredients
            .clone()
            .iter()
            .filter(|i| !i.probably_at_home().unwrap_or(false))
            .flat_map(|i| i.price_total())
            .sum::<f32>();

        Ok(())
    }

    pub fn ingredients(&self) -> Vec<Ingredient> {
        self.ingredients.clone()
    }
}

impl Display for ShoppingList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\ningredients: {}\n\n---------\ntotal: {:.2} €\ntotal without things you probaly have at home: {:.2} €",
            self.ingredients
                .clone()
                    .iter()
                    .map(|i| format!("{}", i))
                    .collect::<Vec<_>>()
                    .join("\n"),
            self.total,
            self.sub_total_without_at_home_items
        )
    }
}
