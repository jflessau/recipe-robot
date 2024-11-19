use leptos::{server, ServerFnError};

#[server]
pub async fn get_ingredients() -> Result<Vec<String>, ServerFnError> {
    Ok(vec![
        "Spaghetti".to_string(),
        "Eggs".to_string(),
        "Bacon".to_string(),
    ])
}
