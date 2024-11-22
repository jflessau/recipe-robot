use crate::{prelude::*, shopping_list::Ingredient};
use leptos::{server, ServerFnError};

#[server]
pub async fn get_ingredients(recipe_text: String) -> Result<Vec<Ingredient>, ServerFnError> {
    use crate::shopping_list::ShoppingList;

    let mut shopping_list = ShoppingList::new(recipe_text, vec!["cheap".to_string()]);

    match shopping_list.find_ingredients().await {
        Ok(_) => Ok(shopping_list.ingredients()),
        Err(e) => Err(ServerFnError::new(e.to_string())),
    }
}
