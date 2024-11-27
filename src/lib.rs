// frontend

pub mod app;
pub mod components;
pub mod error_template;
pub mod prelude;
pub mod views;

// backend

pub mod api;
pub mod shopping_list;
pub mod vendor;

#[cfg(feature = "ssr")]
pub mod fileserv;

#[cfg(feature = "ssr")]
use surrealdb::{engine::any::Any, Surreal};

#[cfg(feature = "ssr")]
#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Surreal<Any>,
    pub jwt_secret: String,
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    _ = console_log::init_with_level(log::Level::Debug);
    leptos::mount_to_body(App);
}
