use super::item::Item;
use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Ingredient {
    #[serde(default = "new_id")]
    pub id: String,
    pub name: String,
    pub probably_at_home: bool,
    pub unit: String,
    pub quantity: usize,

    item: Option<Item>,
    pub item_quantity: Option<usize>,
    #[serde(default)]
    pub alternatives: Vec<Item>,
}

impl Ingredient {
    pub fn select_item(&mut self, id: String, pieces: Option<usize>) {
        if let Some(item) = self.alternatives.iter().find(|i| i.id == id) {
            self.item = Some(item.clone());
            self.item_quantity = pieces;
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

const INGREDIENT_NAME_MAPPINGS: &[(&str, &str)] = &[
    ("Mehl", "Weizenmehl"),
    ("Zucker", "Zucker"),
    ("Salz", "Speisesalz"),
    ("Ei", "Eier"),
];
