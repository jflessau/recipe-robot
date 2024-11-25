mod model;
use model::*;

use super::*;

use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReweConfig {
    pub zip_code: usize,
}

#[derive(Debug)]
pub struct Rewe {
    // config: ReweConfig,
}

impl Rewe {
    pub fn new(_config: ReweConfig) -> Self {
        Self {}
    }

    pub async fn find_items(&self, ingredient: &mut Ingredient) -> Result<(), String> {
        // ask vendor api

        let client = reqwest::Client::new();
        let res = client
            .request(reqwest::Method::GET, "https://shop.rewe.de/api/suggestions")
            .query(&[("q", &ingredient.name())])
            .send()
            .await;

        let Ok(res) = res else {
            error!(
                "failed to search Rewe for {}, error: {:?}",
                ingredient.name(),
                res
            );
            ingredient.set_status(IngredientStatus::ApiSearchFailed {
                error: "Die Anfrage an Rewe ist fehlgeschlagen".to_string(),
            });
            return Err("Die Anfrage an Rewe ist fehlgeschlagen".to_string());
        };

        // deserialize response

        let res = res.json::<ProductListing>().await;
        let Ok(res) = res else {
            error!(
                "failed to search rewe for items {}, deserializing failed, error: {:?}",
                ingredient.name(),
                res
            );
            ingredient.set_status(IngredientStatus::ApiSearchFailed {
                error: "Die Antwort von Rewe konnte nicht verarbeitet werden.".to_string(),
            });
            return Err("Die Antwort von Rewe konnte nicht verarbeitet werden.".to_string());
        };

        info!(
            "searched rewe for items ingredient: {}, res {:#?}",
            ingredient.name(),
            res
        );

        let items = res
            .products
            .into_iter()
            .map(|p| Item {
                id: Uuid::new_v4(),
                name: p.name.clone(),
                quantity: Some(p.grammage),
                price_cent: Some(p.price),
                url: Some(format!("https://www.rewe.de/produkte/{}", p.id)),
                image_url: Some(p.image),
            })
            .collect::<Vec<_>>();

        if items.is_empty() {
            ingredient.set_status(IngredientStatus::NoSearchResults);
        } else {
            ingredient.set_status(IngredientStatus::SearchResults {
                items: items.clone(),
            });
        }

        Ok(())
    }
}
