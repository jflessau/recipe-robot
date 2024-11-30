#[cfg(feature = "ssr")]
use crate::AppState;
use crate::{prelude::*, shopping_list::Ingredient, vendor::Vendor};
use leptos::{server, ServerFnError};

#[cfg(feature = "ssr")]
use super::ai::Ai;
use super::ApiResponse;
#[cfg(feature = "ssr")]
use crate::shopping_list::ShoppingList;

#[cfg(feature = "ssr")]
use super::db::{AuthenticatedUser, IngredientDb, User};

#[server]
pub async fn get_ingredients(
    recipe_text: String,
) -> Result<ApiResponse<Vec<Ingredient>>, ServerFnError> {
    let state = expect_context::<AppState>();
    let Some(user) = expect_context::<Option<AuthenticatedUser>>() else {
        return Err(ServerFnError::new("unauthorized"));
    };

    let shopping_list = ShoppingList::new(recipe_text.clone());
    let ai = Ai::new();

    match ai
        .get_ingredients(&state.db, &user.username, &shopping_list.recipe())
        .await
    {
        Err(err) => {
            error!("failed to get ingredients: {:?}", err);
            Err(ServerFnError::new("failed to get ingredients"))
        }
        Ok(ApiResponse::Err(err)) => Ok(ApiResponse::Err(err)),
        Ok(ApiResponse::Ok(ingredients)) => {
            if let Err(err) =
                User::submit_recipe(&state.db, user.username, recipe_text, ingredients.clone())
                    .await
            {
                error!("failed to submit recipe: {:?}", err);
            }
            Ok(ApiResponse::Ok(ingredients))
        }
    }
}

#[server]
pub async fn get_item_from_vendor(
    ingredient: Ingredient,
    vendor: Vendor,
) -> Result<ApiResponse<Ingredient>, ServerFnError> {
    let state = expect_context::<AppState>();
    let Some(user) = expect_context::<Option<AuthenticatedUser>>() else {
        return Err(ServerFnError::new("unauthorized"));
    };

    let seek_res =
        User::seek_ingredient(&state.db, &user.username, ingredient.clone(), &vendor).await;

    if let Err(e) = seek_res {
        error!("failed to seek ingredient: {:?}", e);
        return Ok(ApiResponse::Err(
            "Die Zutat konnte nicht gefunden werden.".to_string(),
        ));
    };

    let Ok(ingredient_db_id) = seek_res else {
        error!("failed to seek ingredient");
        return Err(ServerFnError::new("failed to seek ingredient"));
    };

    let mut ingredient = ingredient.clone();
    let _r = vendor.find_items(&mut ingredient).await;
    let ai = Ai::new();
    match ai
        .match_item(&state.db, &user.username, &mut ingredient)
        .await
    {
        Ok(ApiResponse::Ok(_)) => {
            if let Err(err) =
                IngredientDb::matches_item(state.db, &ingredient, ingredient_db_id, &vendor).await
            {
                error!("failed to match item: {:?}", err);
                return Err(ServerFnError::new("failed to match item"));
            }
            Ok(ApiResponse::Ok(ingredient))
        }
        Ok(ApiResponse::Err(err)) => Ok(ApiResponse::Err(err)),
        Err(e) => {
            error!("failed to match item: {:?}", e);
            Ok(ApiResponse::Err(
                "Die Zutat konnte nicht gefunden werden.".to_string(),
            ))
        }
    }
}