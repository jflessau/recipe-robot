use crate::prelude::*;
use crate::shopping_list::Ingredient;
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

#[derive(Debug, Clone, Deserialize, Serialize)]
struct IngredientDb {
    name: String,
    probably_at_home: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Requires {
    r#in: Thing,
    out: Thing,
    quantity: usize,
    unit: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    id: RecordId,
    password_hash: String,
}

impl User {
    pub async fn join(db: Surreal<Any>, invite_code: String) -> Result<(String, String)> {
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

    pub async fn login(db: Surreal<Any>, username: String, password: String) -> Result<String> {
        let Some(user): Option<User> = db.select(("user", &username)).await? else {
            bail!("User not found")
        };

        if !verify(&password, &user.password_hash)? {
            bail!("Invalid password")
        }

        Ok(user.id.key().to_string())
    }

    pub async fn submit_recipe(
        db: Surreal<Any>,
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

        // insert cash flow

        let input_characters = recipe_text.len();
        let output_characters = serde_json::to_string(&ingredients)?.len();
        let cash_flow = CashFlow {
            amount: CashFlow::input_token(input_characters)
                + CashFlow::output_token(output_characters),
            origin: CashFlowOrigin::Ai,
        };
        let cash_flow_id = Uuid::new_v4().to_string().replace("-", "");
        let Some(_cash_flow) = db
            .upsert::<Option<CashFlow>>(("cash_flow", &cash_flow_id))
            .content(cash_flow.clone())
            .await?
        else {
            bail!("Fails to create cash_flow.")
        };

        // relate cash flow to user

        let _r = db
            .insert::<Vec<Relation>>("generates")
            .relation(Relation {
                r#in: thing(&format!("user:{username}"))?,
                out: thing(&format!("cash_flow:{cash_flow_id}"))?,
            })
            .await?;

        // insert ingredients and relate them to recipe

        for ingredient in ingredients {
            let ingredient_db = IngredientDb {
                name: ingredient.name.clone(),
                probably_at_home: ingredient.probably_at_home,
            };

            let ingredient_id = Uuid::new_v4().to_string().replace("-", "");
            let Some(_ingredient) = db
                .upsert::<Option<IngredientDb>>(("ingredient", &ingredient_id))
                .content(ingredient_db.clone())
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
    pub fn input_token(character_amount: usize) -> i64 {
        let cents = character_amount as f64 / 3.6 / 1_000_000.0 * 15.0;
        let micro_dollar = cents * 10_000.0;
        -micro_dollar as i64
    }

    pub fn output_token(character_amount: usize) -> i64 {
        let cents = character_amount as f64 / 3.6 / 1_000_000.0 * 60.0;
        let micro_dollar = cents * 10_000.0;
        -micro_dollar as i64
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum CashFlowOrigin {
    #[serde(rename = "ai")]
    Ai,
    #[serde(rename = "private_assets")]
    PrivateAssets,
    #[serde(rename = "donation")]
    Donation,
}
