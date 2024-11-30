use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: RecordId,
    pub password_hash: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Relation {
    pub r#in: Thing,
    pub out: Thing,
}
