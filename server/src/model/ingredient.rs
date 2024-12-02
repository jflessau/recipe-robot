use super::item::Item;
use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Ingredient {
    #[serde(default = "new_id")]
    pub id: String,
    pub name: String,
    pub probably_at_home: bool,
    pub unit: String,
    pub quantity: f64,

    item: Option<Item>,
    #[serde(default)]
    pub item_quantity: i64,
    #[serde(default)]
    pub alternatives: Vec<Item>,
}

impl Ingredient {
    pub fn item(&self) -> Option<Item> {
        self.item.clone()
    }

    pub fn select_item(&mut self, id: String, pieces: Option<i64>) {
        if let Some(item) = self.alternatives.iter().find(|i| i.id == id) {
            self.item = Some(item.clone());
            self.item_quantity = pieces.unwrap_or(1);
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
    ("Eiweiß", "Eier"),
    ("Eigelb", "Eier"),
];
