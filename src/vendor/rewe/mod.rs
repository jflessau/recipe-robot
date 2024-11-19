mod model;
use model::*;

use super::*;

use anyhow::Result;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Rewe {
    // config: ReweConfig,
    client: reqwest::Client,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReweConfig {
    pub zip_code: usize,
}

impl Vendor for Rewe {
    type Config = ReweConfig;

    async fn new(_config: ReweConfig) -> Result<Self> {
        let client = reqwest::Client::new();
        client
            .request(reqwest::Method::GET, "https://shop.rewe.de/api/suggestions")
            .query(&[("q", "milch")])
            .send()
            .await?
            .json::<ProductListing>()
            .await?;

        Ok(Rewe {
            // config,
            client: reqwest::Client::new(),
        })
    }

    fn name(&self) -> String {
        "Rewe".to_string()
    }

    async fn search_for_items(&self, ingredient: Ingredient) -> Result<Vec<Item>> {
        let res = self
            .client
            .request(reqwest::Method::GET, "https://shop.rewe.de/api/suggestions")
            .query(&[("q", &ingredient.name())])
            .send()
            .await?
            .json::<ProductListing>()
            .await?;

        let items = res
            .products
            .into_iter()
            .map(|p| Item {
                name: p.name.clone(),
                quantity: Some(p.grammage),
                price_cent: Some(p.price),
                url: Some(p.url),
                image_url: Some(p.image),
            })
            .collect::<Vec<_>>();

        Ok(items)
    }
}
