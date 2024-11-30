use crate::api::ai::AiUsage;
use crate::prelude::*;
use crate::shopping_list::{Ingredient, IngredientStatus};
use crate::vendor::{Item, Vendor};
use bcrypt::{hash, verify, DEFAULT_COST};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rnglib::{Language, RNG};
use surrealdb::{
    engine::any::Any,
    sql::{thing, Thing},
    RecordId, Surreal,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthenticatedUser {
    pub username: String,
}

impl AuthenticatedUser {
    pub fn new(username: String) -> Self {
        AuthenticatedUser { username }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Relation {
    r#in: Thing,
    out: Thing,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Recipe {
    text: String,
}

impl From<Ingredient> for IngredientDb {
    fn from(ingredient: Ingredient) -> Self {
        IngredientDb {
            name: ingredient.name,
            probably_at_home: ingredient.probably_at_home,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Requires {
    r#in: Thing,
    out: Thing,
    quantity: usize,
    unit: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Seeks {
    r#in: Thing,
    out: Thing,
    quantity: usize,
    unit: String,
    vendor: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Matches {
    r#in: Thing,
    out: Thing,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ItemDb {
    name: String,
    vendor: String,
    price_cent: Option<i64>,
    quantity: Option<String>,
    url: Option<String>,
    image_url: Option<String>,
}

impl From<(Item, &Vendor)> for ItemDb {
    fn from((item, vendor): (Item, &Vendor)) -> Self {
        ItemDb {
            name: item.name,
            vendor: vendor.to_string(),
            price_cent: item.price_cent.map(|p| p as i64),
            quantity: item.quantity,
            url: item.url,
            image_url: item.image_url,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    id: RecordId,
    password_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Invite {
    initial_charges: usize,
    used_charges: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CashFlow {
    pub amount: i64, // micro dollar (- = out, + = in)
    pub origin: CashFlowOrigin,
}

impl CashFlow {
    pub async fn attribute_ai_costs(
        db: &Surreal<Any>,
        username: &String,
        ai_usages: Vec<AiUsage>,
    ) -> Result<()> {
        let cash_flows = ai_usages
            .iter()
            .map(|usage| {
                let amount = usage.costs_in_micro_dollar() as i64;
                let origin = CashFlowOrigin::from(usage);
                CashFlow { amount, origin }
            })
            .collect::<Vec<_>>();

        // insert cash flows

        for cash_flow in cash_flows {
            // create cash_flow

            let cash_flow_id = Uuid::new_v4().to_string().replace("-", "");
            let _r: Option<CashFlow> = db
                .create(("cash_flow", &cash_flow_id))
                .content(cash_flow)
                .await?;

            // relate user to cash_flow

            let _r = db
                .insert::<Vec<Relation>>("generates")
                .relation(Relation {
                    r#in: thing(&format!("user:{username}"))?,
                    out: thing(&format!("cash_flow:{cash_flow_id}"))?,
                })
                .await?;
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum CashFlowOrigin {
    #[serde(rename = "ai_input_token")]
    AiInputToken,
    #[serde(rename = "ai_output_token")]
    AiOutputToken,
    #[serde(rename = "private_assets")]
    PrivateAssets,
    #[serde(rename = "donation")]
    Donation,
}

impl From<&AiUsage> for CashFlowOrigin {
    fn from(usage: &AiUsage) -> Self {
        match usage {
            AiUsage::InputToken(_) => CashFlowOrigin::AiInputToken,
            AiUsage::OutputToken(_) => CashFlowOrigin::AiOutputToken,
        }
    }
}

impl User {
    pub async fn join(db: &Surreal<Any>, invite_code: String) -> Result<(String, String)> {
        let Some(mut invite): Option<Invite> = db.select(("invite", &invite_code)).await? else {
            bail!("Invalid invite code")
        };

        info!("Invite: {:?}", invite);
        if invite.used_charges == invite.initial_charges {
            bail!("Invite code is already used.")
        }

        invite.used_charges += 1;
        let Some(_invite) = db
            .upsert::<Option<Invite>>(("invite", &invite_code))
            .content(invite.clone())
            .await?
        else {
            bail!("Invalid invite code")
        };

        let password: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(24)
            .map(char::from)
            .collect();

        let password_hash = hash(&password, DEFAULT_COST)?;
        let rng = RNG::from(&Language::Fantasy);
        let username = rng.generate_name_by_count(3).to_lowercase();

        let user = User {
            id: ("user", username.clone()).into(),
            password_hash,
        };

        let Some(_user) = db
            .upsert::<Option<User>>(user.id.clone())
            .content(user.clone())
            .await?
        else {
            bail!("Fails to create user.")
        };

        let r = db
            .insert::<Vec<Relation>>("spawns")
            .relation(Relation {
                r#in: thing(&format!("invite:{invite_code}"))?,
                out: thing(&format!("user:{username}"))?,
            })
            .await;

        info!("r: {:?}", r);

        Ok((username, password))
    }

    pub async fn login(db: &Surreal<Any>, username: String, password: String) -> Result<String> {
        let Some(user): Option<User> = db.select(("user", &username)).await? else {
            bail!("User not found")
        };

        if !verify(&password, &user.password_hash)? {
            bail!("Invalid password")
        }

        Ok(user.id.key().to_string())
    }

    pub async fn submit_recipe(
        db: &Surreal<Any>,
        username: String,
        recipe_text: String,
        ingredients: Vec<Ingredient>,
    ) -> Result<()> {
        let Some(_user): Option<User> = db.select(("user", &username)).await? else {
            bail!("User not found")
        };

        // insert recipe

        let recipe = Recipe {
            text: recipe_text.clone(),
        };
        let recipe_id = Uuid::new_v4().to_string().replace("-", "");
        let Some(_recipe) = db
            .upsert::<Option<Recipe>>(("recipe", &recipe_id))
            .content(recipe.clone())
            .await?
        else {
            bail!("Fails to create recipe.")
        };

        // relate recipe to user

        let _r = db
            .insert::<Vec<Relation>>("submits")
            .relation(Relation {
                r#in: thing(&format!("user:{username}"))?,
                out: thing(&format!("recipe:{recipe_id}"))?,
            })
            .await?;

        // insert ingredients and relate them to recipe

        for ingredient in ingredients {
            let ingredient_db: IngredientDb = ingredient.clone().into();

            let ingredient_id = Uuid::new_v4().to_string().replace("-", "");
            let Some(_ingredient) = db
                .upsert::<Option<IngredientDb>>(("ingredient", &ingredient_id))
                .content(ingredient_db)
                .await?
            else {
                bail!("Fails to create ingredient.")
            };

            let requires = Requires {
                r#in: thing(&format!("recipe:{recipe_id}"))?,
                out: thing(&format!("ingredient:{ingredient_id}"))?,
                quantity: ingredient.quantity,
                unit: ingredient.unit.clone(),
            };

            let _r = db
                .insert::<Vec<Requires>>("requires")
                .relation(requires)
                .await?;
        }

        Ok(())
    }

    pub async fn seek_ingredient(
        db: &Surreal<Any>,
        username: &String,
        ingredient: Ingredient,
        vendor: &Vendor,
    ) -> Result<String> {
        let Some(_user): Option<User> = db.select(("user", username)).await? else {
            bail!("User not found")
        };

        // insert ingredient

        let ingredient_db: IngredientDb = ingredient.clone().into();
        let ingredient_id = Uuid::new_v4().to_string().replace("-", "");
        let Some(_ingredient) = db
            .upsert::<Option<IngredientDb>>(("ingredient", &ingredient_id))
            .content(ingredient_db)
            .await?
        else {
            bail!("Fails to create ingredient.")
        };

        // relate user to ingredient

        let _r = db
            .insert::<Vec<Relation>>("seeks")
            .relation(Seeks {
                r#in: thing(&format!("user:{username}"))?,
                out: thing(&format!("ingredient:{ingredient_id}"))?,
                quantity: ingredient.quantity,
                unit: ingredient.unit.clone(),
                vendor: vendor.to_string(),
            })
            .await?;

        Ok(ingredient_id)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IngredientDb {
    name: String,
    probably_at_home: bool,
}

impl IngredientDb {
    pub async fn matches_item(
        db: Surreal<Any>,
        ingredient: &Ingredient,
        ingredient_db_id: String,
        vendor: &Vendor,
    ) -> Result<()> {
        let IngredientStatus::Matched { item, .. } = ingredient.status() else {
            warn!("tried to relate ingredient without item to item");
            return Ok(());
        };

        // create item

        let item_id = item.id().to_string().replace("-", "");
        let item_db: ItemDb = (item.clone(), vendor).into();
        let Some(_item) = db
            .upsert::<Option<ItemDb>>(("item", &item_id))
            .content(item_db)
            .await?
        else {
            bail!("Fails to create item.")
        };

        // relate ingredient to item

        let _r = db
            .insert::<Vec<Matches>>("matches")
            .relation(Matches {
                r#in: thing(&format!("item:{item_id}"))?,
                r#out: thing(&format!("ingredient:{ingredient_db_id}"))?,
            })
            .await?;

        Ok(())
    }
}
