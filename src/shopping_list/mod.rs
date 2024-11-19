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
    ingredients: Option<Vec<Ingredient>>,
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
            ingredients: None,
            themes,

            vendor,

            ai,

            sub_total_without_at_home_items: 0.0,
            total: 0.0,
        }
    }

    pub async fn make_progress(&mut self) -> Result<()> {
        let vendor = match &self.vendor {
            VendorSelect::Rewe { config } => Rewe::new(config.clone()).await,
        };

        let Ok(vendor) = vendor else {
            panic!("failed to talk to vendor, error: {vendor:#?}");
        };

        info!("🤖 list ingredients...");
        let ingredients = self
            .ai
            .get_ingredients(&self.recipe)
            .await
            .context("failed to get ingredients")?;

        info!(
            "ingredients: {:?}",
            ingredients
                .iter()
                .map(|i| i.name().clone())
                .collect::<Vec<_>>()
        );

        // search for items

        let mut res = vec![];

        for mut ingredient in ingredients {
            info!(
                "🔍 search {} for ingredient {}",
                vendor.name(),
                ingredient.name()
            );

            // get items

            match vendor.search_for_items(ingredient.clone()).await {
                Err(err) => {
                    ingredient.set_status(IngredientStatus::ApiSearchFailed {
                        error: format!("{err:?}"),
                    });
                }
                Ok(items) => {
                    ingredient.set_status(IngredientStatus::SearchResults { items });
                }
            };

            // match items

            info!("🤖 use ai to match items");
            res.push(
                vendor
                    .match_item(ingredient, &self.themes, &self.ai)
                    .await
                    .context("failed to match item")?,
            );
        }

        self.ingredients = Some(res.clone());

        // calculate totals

        self.total = res.iter().flat_map(|i| i.price_total()).sum::<f32>();
        self.sub_total_without_at_home_items = res
            .iter()
            .filter(|i| !i.probably_at_home().unwrap_or(false))
            .flat_map(|i| i.price_total())
            .sum::<f32>();

        Ok(())
    }

    pub fn ingredients(&self) -> Vec<Ingredient> {
        self.ingredients.clone().unwrap_or_default()
    }
}

impl Display for ShoppingList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\ningredients: {}\n\n---------\ntotal: {:.2} €\ntotal without things you probaly have at home: {:.2} €",
            self.ingredients
                .clone()
                .map(|i| i
                    .iter()
                    .map(|i| format!("{}", i))
                    .collect::<Vec<_>>()
                    .join("\n"))
                .unwrap_or("none".to_string()),
            self.total,
            self.sub_total_without_at_home_items
        )
    }
}
