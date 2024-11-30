use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecipeIn {
    text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IngredientsOut {
    ingredients: Vec<Ingredient>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IngredientMatchIn {
    ingredient: Ingredient,
    vendor: Vendor,
}

pub async fn get_recipe_ingredients(
    authenticated_user: AuthenticatedUser,
    Extension(state): Extension<AppState>,
    Json(payload): Json<RecipeIn>,
) -> Result<Json<IngredientsOut>, Error> {
    let ai = Ai::new();
    let ingredients = ai
        .get_ingredients(&state.db, &authenticated_user.username, &payload.text)
        .await?;

    Ok(Json(IngredientsOut { ingredients }))
}

pub async fn get_items_for_ingredient(
    authenticated_user: AuthenticatedUser,
    Extension(state): Extension<AppState>,
    Json(payload): Json<IngredientMatchIn>,
) -> Result<Json<Ingredient>, Error> {
    let mut ingredient = payload.ingredient.clone();

    payload.vendor.find_items(&mut ingredient).await?;

    let ai = Ai::new();
    ai.match_item(&state.db, &authenticated_user.username, &mut ingredient)
        .await?;

    Ok(Json(ingredient))
}
