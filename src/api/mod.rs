use crate::prelude::*;
use leptos::{server, ServerFnError};

#[server]
pub async fn get_ingredients(recipe_text: String) -> Result<Vec<String>, ServerFnError> {
    use crate::shopping_list::ShoppingList;

    info!("get_ingredients: recipe_text: {:?}", recipe_text);

    let mut shopping_list = ShoppingList::new(recipe_text, vec!["cheap".to_string()]);

    match shopping_list.find_ingredients().await {
        Ok(_) => {
            let ingredients = shopping_list
                .ingredients()
                .into_iter()
                .map(|i| i.name().to_string())
                .collect::<Vec<_>>();

            Ok(ingredients)
        }
        Err(e) => Err(ServerFnError::new(e.to_string())),
    }
}
