use super::*;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Ingredient {
    name: String,
    #[allow(dead_code)]
    name_og: String,
    #[allow(dead_code)]
    probably_at_home: Option<bool>,
    #[serde(rename = "amount")]
    amount_required: String,

    #[serde(default)]
    status: IngredientStatus,
}

impl Ingredient {
    pub fn status(&self) -> &IngredientStatus {
        &self.status
    }

    pub fn set_status(&mut self, status: IngredientStatus) {
        self.status = status;
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn probably_at_home(&self) -> Option<bool> {
        self.probably_at_home
    }

    pub fn price_total(&self) -> Option<f32> {
        match &self.status {
            IngredientStatus::Matched { item, pieces, .. } => Some(item.price_total(*pieces)),
            _ => None,
        }
    }
}

impl Display for Ingredient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}{}",
            self.name,
            self.status,
            if self.probably_at_home.unwrap_or(false) {
                " ℹ️ you probably have this at home"
            } else {
                ""
            },
        )
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub enum IngredientStatus {
    #[default]
    Unchecked,
    Checking,
    ApiSearchFailed {
        error: String,
    },
    NoSearchResults,
    SearchResults {
        items: Vec<Item>,
    },
    AiRefusedToSelectItem {
        alternatives: Vec<Item>,
    },
    AiSelectedInvalidItem {
        alternatives: Vec<Item>,
    },
    Alternative {
        original: String,
        selected: Item,
        alternatives: Vec<Item>,
    },
    Matched {
        item: Item,
        pieces: usize,
        alternatives: Vec<Item>,
    },
}

impl Display for IngredientStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IngredientStatus::Unchecked => write!(f, "⏳"),
            IngredientStatus::Checking => write!(f, "🔍"),
            IngredientStatus::ApiSearchFailed { error } => {
                write!(f, "⚠️ search failed: {error}")
            }
            IngredientStatus::NoSearchResults => write!(f, "◌"),
            IngredientStatus::SearchResults { items } => {
                write!(f, "🛒 {} items found", items.len())
            }
            IngredientStatus::AiRefusedToSelectItem { alternatives } => {
                write!(
                    f,
                    "🤖 🤷 {} items found, but AI thinks nothing really matches",
                    alternatives.len()
                )
            }
            IngredientStatus::AiSelectedInvalidItem { alternatives } => {
                write!(
                    f,
                    "🤖 ⚠️ {} items found, but AI fails to select one",
                    alternatives.len()
                )
            }
            IngredientStatus::Alternative { .. } => write!(f, "↩️ alternative selected"),
            IngredientStatus::Matched { item, pieces, .. } => {
                write!(
                    f,
                    "✅ {}x {} (💰 {} €)",
                    pieces,
                    item.name(),
                    item.price_total(*pieces)
                )
            }
        }
    }
}
