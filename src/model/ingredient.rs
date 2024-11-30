use super::item::Item;
use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Ingredient {
    pub id: String,
    pub name: String,
    pub probably_at_home: bool,
    pub unit: String,
    pub quantity: usize,

    #[serde(default)]
    pub status: IngredientStatus,
}

impl Ingredient {
    pub fn set_item_pieces(&mut self, p: usize) {
        if !(0..100).contains(&p) {
            return;
        }

        if let IngredientStatus::Matched { pieces, .. } = &mut self.status {
            *pieces = p;
        }
    }

    pub fn select_item(&mut self, id: String, pieces: Option<usize>) {
        if let IngredientStatus::SearchResults { items }
        | IngredientStatus::AiFailsToSelectItem {
            alternatives: items,
        }
        | IngredientStatus::Matched {
            alternatives: items,
            ..
        } = &mut self.status
        {
            if let Some(item) = items.iter().find(|i| i.id == id) {
                self.status = IngredientStatus::Matched {
                    item: item.clone(),
                    pieces: pieces.unwrap_or(1),
                    alternatives: items.clone(),
                };
            }
        }
    }

    pub fn enrich(&mut self) {
        self.name = INGREDIENT_NAME_MAPPINGS
            .iter()
            .find(|(n, _)| n == &self.name)
            .map(|(_, m)| m.to_string())
            .unwrap_or_else(|| self.name.clone());
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, Hash)]
pub enum IngredientStatus {
    #[default]
    Unchecked,
    ApiSearchFailed {
        error: String,
    },
    NoSearchResults,
    SearchResults {
        items: Vec<Item>,
    },
    AiFailsToSelectItem {
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
            IngredientStatus::ApiSearchFailed { error } => {
                write!(f, "⚠️ search failed: {error}")
            }
            IngredientStatus::NoSearchResults => write!(f, "◌"),
            IngredientStatus::SearchResults { items } => {
                write!(f, "🛒 {} items found", items.len())
            }
            IngredientStatus::AiFailsToSelectItem { alternatives } => {
                write!(
                    f,
                    "🤖 ❌ AI failed to select an item, {} items found",
                    alternatives.len()
                )
            }
            IngredientStatus::Matched { item, pieces, .. } => {
                write!(
                    f,
                    "✅ {}x {} (💰 {} €)",
                    pieces,
                    item.name,
                    item.price_total(*pieces)
                )
            }
        }
    }
}

const INGREDIENT_NAME_MAPPINGS: &[(&str, &str)] = &[
    ("Mehl", "Weizenmehl"),
    ("Zucker", "Zucker"),
    ("Salz", "Speisesalz"),
    ("Ei", "Eier"),
];
