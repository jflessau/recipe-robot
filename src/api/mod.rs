#[cfg(feature = "ssr")]
mod ai;
mod auth;
#[cfg(feature = "ssr")]
mod db;
mod shopping_list;

#[cfg(feature = "ssr")]
pub use auth::Claims;
#[cfg(feature = "ssr")]
pub use db::AuthenticatedUser;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ApiResponse<T> {
    Ok(T),
    Err(String),
}

pub use auth::{authorized, join, login, logout};
pub use shopping_list::{get_ingredients, get_item_from_vendor};
