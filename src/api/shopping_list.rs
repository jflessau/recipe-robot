#[cfg(feature = "ssr")]
use crate::AppState;
use crate::{prelude::*, shopping_list::Ingredient, vendor::Vendor};
use leptos::{server, ServerFnError};

#[cfg(feature = "ssr")]
use super::ai::Ai;
#[cfg(feature = "ssr")]
use crate::shopping_list::ShoppingList;

#[cfg(feature = "ssr")]
use super::db::{AuthenticatedUser, User};

#[server]
pub async fn get_ingredients(recipe_text: String) -> Result<Vec<Ingredient>, ServerFnError> {
    let state = expect_context::<AppState>();
    let Some(user) = expect_context::<Option<AuthenticatedUser>>() else {
        return Err(ServerFnError::new("unauthorized"));
    };

    let shopping_list = ShoppingList::new(recipe_text.clone());

    let ai = Ai::new();

    let Ok(ingredients) = ai.get_ingredients(&shopping_list.recipe()).await else {
        return Err(ServerFnError::new("Die AI konnte keine Zutaten finden."));
    };

    match User::submit_recipe(state.db, user.username, recipe_text, ingredients.clone()).await {
        Ok(_) => Ok(ingredients),
        Err(e) => {
            error!("failed to submit recipe: {:?}", e);
            return Err(ServerFnError::new("failed to submit recipe"));
        }
    }
}

#[server]
pub async fn get_item_from_vendor(
    ingredient: Ingredient,
    vendor: Vendor,
) -> Result<Ingredient, ServerFnError> {
    let mut ingredient = ingredient.clone();
    let _r = vendor.find_items(&mut ingredient).await;
    let ai = Ai::new();
    match ai.match_item(&mut ingredient).await {
        Ok(_) => {}
        Err(e) => {
            error!("failed to match item: {:?}", e);
            return Err(ServerFnError::new("failed to match item"));
        }
    }

    Ok(ingredient)
}
