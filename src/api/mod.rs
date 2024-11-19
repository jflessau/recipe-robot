use leptos::{server, ServerFnError};

#[server]
pub async fn get_ingredients() -> Result<Vec<String>, ServerFnError> {
    use crate::shopping_list::ShoppingList;

    let mut shopping_list = ShoppingList::new(
        r#"
            Quick Pancake Recipe
            Ingredients:
            1 cup flour
            1 tbsp sugar
        "#
        .to_string(),
        vec!["cheap".to_string()],
    );

    match shopping_list.make_progress().await {
        Ok(_) => {
            let ingredients = shopping_list
                .ingredients()
                .into_iter()
                .map(|i| format!("{}", i.name()))
                .collect::<Vec<_>>();

            return Ok(ingredients);
        }
        Err(e) => {
            return Err(ServerFnError::new(e.to_string()));
        }
    }
}
