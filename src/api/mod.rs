mod ai;

use crate::{prelude::*, shopping_list::Ingredient, vendor::Vendor};

#[cfg(feature = "ssr")]
use crate::shopping_list::ShoppingList;

#[cfg(feature = "ssr")]
use ai::Ai;
use leptos::{server, ServerFnError};

#[server]
pub async fn get_ingredients(recipe_text: String) -> Result<Vec<Ingredient>, ServerFnError> {
    let shopping_list = ShoppingList::new(recipe_text);

    let ai = Ai::new();

    let Ok(ingredients) = ai.get_ingredients(&shopping_list.recipe()).await else {
        return Err(ServerFnError::new("Die AI konnte keine Zutaten finden."));
    };

    Ok(ingredients)
}

#[server]
pub async fn get_item_from_vendor(
    ingredient: Ingredient,
    vendor: Vendor,
) -> Result<Ingredient, ServerFnError> {
    let mut ingredient = ingredient.clone();
    let _r = vendor.find_items(&mut ingredient).await;
    let ai = Ai::new();
    match ai
        .match_item(&mut ingredient, &vec!["cheap".to_string()])
        .await
    {
        Ok(_) => {}
        Err(e) => {
            error!("failed to match item: {:?}", e);
            return Err(ServerFnError::new("failed to match item"));
        }
    }

    Ok(ingredient)
}
