mod model;
use model::*;

use crate::prelude::*;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReweConfig {
    pub zip_code: usize,
}

#[derive(Debug)]
pub struct Rewe {
    zip_code: String,
}

impl Rewe {
    pub fn new(zip_code: String) -> Self {
        Self { zip_code }
    }

    pub async fn find_items(&self, ingredient: &mut Ingredient) -> Result<(), Error> {
        // ask vendor api

        let client = reqwest::Client::new();
        let res = client
            .request(reqwest::Method::GET, "https://shop.rewe.de/api/products")
            .query(&[
                ("objectsPerPage", "16"),
                ("page", "1"),
                ("search", &ingredient.name),
                ("sorting", "RELEVANCE_DESC"),
                ("serviceTypes", "PICKUP"),
                ("market", "540528"),
                ("debug", "false"),
                ("autocorrect", "true"),
            ])
            .send()
            .await;

        let Ok(res) = res else {
            error!(
                "failed to search Rewe for {}, error: {:?}",
                ingredient.name, res
            );
            return Err(Error::InternalServer);
        };

        // deserialize response

        let res = res.json::<ProductSearchResult>().await;
        let Ok(res) = res else {
            error!(
                "failed to search rewe for items {}, deserializing failed, error: {:?}",
                ingredient.name, res
            );
            return Err(Error::InternalServer);
        };

        let items = res
            .products
            .products
            .into_iter()
            .map(|p| {
                let pricing = p
                    .meta
                    .articles
                    .into_iter()
                    .next()
                    .map(|a| a.article.listing.pricing);

                Item {
                    id: new_id(),
                    name: p.name.clone(),
                    grammage: pricing.clone().map(|p| p.grammage),
                    price_cent: pricing.clone().map(|p| p.current_retail_price),
                    url: Some(format!("https://www.rewe.de/produkte/{}", p.id)),
                    image_url: p.media.images.into_iter().next().map(|i| i.links.link.href),
                }
            })
            .collect::<Vec<_>>();

        ingredient.alternatives = items;

        Ok(())
    }
}
