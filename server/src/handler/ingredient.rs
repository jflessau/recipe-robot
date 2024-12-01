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
    // vendor: Vendor,
}

pub async fn get_recipe_ingredients(
    authenticated_user: AuthenticatedUser,
    Extension(state): Extension<AppState>,
    Json(payload): Json<RecipeIn>,
) -> Result<Json<IngredientsOut>, Error> {
    // store recipe

    let recipe = Recipe {
        text: payload.text.clone(),
    };
    let recipe_id = new_id();
    let Some(_r): Option<Recipe> = state
        .db
        .create(("recipe", &recipe_id))
        .content(recipe)
        .await?
    else {
        error!("failed to store recipe");
        return Err(Error::InternalServer);
    };

    let username = &authenticated_user.username;
    let Some(_r) = state
        .db
        .insert::<Vec<Relation>>("submits")
        .relation(Relation {
            r#in: thing(&format!("user:{username}"))?,
            out: thing(&format!("recipe:{recipe_id}"))?,
        })
        .await?
        .first()
    else {
        error!("failed to store user -> recipe relation");
        return Err(Error::InternalServer);
    };

    // get ingredients

    let ai = Ai::default();
    let ingredients = ai
        .get_ingredients(&state.db, &authenticated_user.username, &payload.text)
        .await?;

    // store ingredients and relate them to recipe

    for ingredient in &ingredients {
        let ingredient_db: IngredientDb = ingredient.clone().into();

        let ingredient_id = new_id();
        let Some(_ingredient) = state
            .db
            .upsert::<Option<IngredientDb>>(("ingredient", &ingredient_id))
            .content(ingredient_db)
            .await?
        else {
            error!("failed to store ingredient");
            return Err(Error::InternalServer);
        };

        let requires = Requires {
            r#in: thing(&format!("recipe:{recipe_id}"))?,
            out: thing(&format!("ingredient:{ingredient_id}"))?,
            quantity: ingredient.quantity,
            unit: ingredient.unit.clone(),
        };

        let Some(_r) = state
            .db
            .insert::<Vec<Requires>>("requires")
            .relation(requires)
            .await?
            .first()
        else {
            error!("failed to store recipe -> ingredient relation");
            return Err(Error::InternalServer);
        };
    }

    Ok(Json(IngredientsOut { ingredients }))
}

pub async fn get_items(
    authenticated_user: AuthenticatedUser,
    Extension(state): Extension<AppState>,
    Json(payload): Json<IngredientMatchIn>,
) -> Result<Json<Ingredient>, Error> {
    let mut ingredient = payload.ingredient.clone();
    let vendor = Vendor::Rewe {
        zip_code: "1237".to_string(),
    }; // TODO get vendor from payload

    // insert ingredient

    let ingredient_db: IngredientDb = ingredient.clone().into();
    let ingredient_id = new_id();
    let Some(_ingredient) = state
        .db
        .upsert::<Option<IngredientDb>>(("ingredient", &ingredient_id))
        .content(ingredient_db)
        .await?
    else {
        error!("failed to store ingredient");
        return Err(Error::InternalServer);
    };

    // relate user to ingredient

    let username = &authenticated_user.username;
    let Some(_r) = state
        .db
        .insert::<Vec<Relation>>("seeks")
        .relation(Seeks {
            r#in: thing(&format!("user:{username}"))?,
            out: thing(&format!("ingredient:{ingredient_id}"))?,
            quantity: ingredient.quantity,
            unit: ingredient.unit.clone(),
            vendor: vendor.name(),
        })
        .await?
        .first()
    else {
        error!("failed to store user -> ingredient relation");
        return Err(Error::InternalServer);
    };

    // find items at vendor

    vendor.find_items(&mut ingredient).await?;

    // match item to ingredient

    let ai = Ai::default();
    ai.match_item(&state.db, &authenticated_user.username, &mut ingredient)
        .await?;

    // store item

    if let Some(item) = &ingredient.item() {
        let item_id = item.id.to_string().replace("-", "");
        let item_db: ItemDb = (item.clone(), &vendor).into();
        let Some(_item) = state
            .db
            .upsert::<Option<ItemDb>>(("item", &item_id))
            .content(item_db)
            .await?
        else {
            error!("failed to store item");
            return Err(Error::InternalServer);
        };

        // relate ingredient to item

        let Some(_r) = state
            .db
            .insert::<Vec<Relation>>("matches")
            .relation(Relation {
                r#in: thing(&format!("item:{item_id}"))?,
                r#out: thing(&format!("ingredient:{ingredient_id}"))?,
            })
            .await?
            .first()
        else {
            error!("failed to store item -> ingredient relation");
            return Err(Error::InternalServer);
        };
    }

    Ok(Json(ingredient))
}
