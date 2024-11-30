mod ingredient;

pub use ingredient::{Ingredient, IngredientStatus};

use crate::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ShoppingList {
    recipe: String,
    ingredients: Vec<Ingredient>,
}

impl ShoppingList {
    pub fn new(recipe: String) -> Self {
        Self {
            recipe,
            ingredients: vec![],
        }
    }

    pub fn recipe(&self) -> String {
        self.recipe.clone()
    }

    pub fn ingredients(&self) -> Vec<Ingredient> {
        self.ingredients.clone()
    }
}

impl Display for ShoppingList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\ningredients: {}",
            self.ingredients
                .clone()
                .iter()
                .map(|i| format!("{}", i))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}
