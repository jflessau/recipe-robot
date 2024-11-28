use crate::prelude::*;
use bcrypt::{hash, verify, DEFAULT_COST};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rnglib::{Language, RNG};
use surrealdb::{
    engine::any::Any,
    sql::{thing, Thing},
    Datetime as SurrealDateTime, RecordId, Surreal,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthenticatedUser {
    username: String,
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
struct Relation2 {
    r#in: RecordId,
    out: RecordId,
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
        let rng = RNG::try_from(&Language::Fantasy)?;
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
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Invite {
    initial_charges: usize,
    used_charges: usize,
}
