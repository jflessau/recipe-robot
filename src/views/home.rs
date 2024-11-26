use crate::components::{
    loading_indicator::LoadingIndicator, recipe_input::View as RecipeInput,
    shopping_list::ShoppingList,
};
use crate::{
    prelude::*,
    shopping_list::{Ingredient, IngredientStatus},
    vendor::Item,
};

#[derive(Debug, Clone)]
pub enum State {
    Error {
        recipe_text: String,
        error: String,
    },
    RecipeInput {
        recipe_text: String,
    },
    FindIngredients {
        recipe_text: String,
    },
    ShoppingList {
        recipe_text: String,
        ingredients: Vec<Ingredient>,
    },
}

impl State {
    pub fn recipe_text(&self) -> String {
        match self {
            State::RecipeInput { recipe_text } => recipe_text.clone(),
            State::FindIngredients { recipe_text } => recipe_text.clone(),
            State::Error { recipe_text, .. } => recipe_text.clone(),
            State::ShoppingList { recipe_text, .. } => recipe_text.clone(),
        }
    }

    pub fn show_recipe_input(&self) -> bool {
        matches!(self, State::RecipeInput { .. })
    }

    pub fn show_loading_indicator(&self) -> bool {
        matches!(self, State::FindIngredients { .. })
    }

    pub fn error_msg(&self) -> Option<String> {
        match self {
            State::Error { error, .. } => Some(error.clone()),
            _ => None,
        }
    }

    pub fn ingredients(&self) -> Option<&Vec<Ingredient>> {
        match self {
            State::ShoppingList { ingredients, .. } => Some(ingredients),
            _ => None,
        }
    }

    pub fn set_recipe_text(&mut self, recipe_text: String) -> Self {
        State::RecipeInput { recipe_text }
    }

    pub fn find_ingredients(&mut self) -> Self {
        match self {
            State::RecipeInput { recipe_text } => {
                if recipe_text.trim().is_empty() {
                    log::warn!("find_ingredients called with empty recipe text");
                    return State::Error {
                        recipe_text: recipe_text.clone(),
                        error: "Der Text vom Rezept darf nicht leer sein.".to_string(),
                    };
                }
                State::FindIngredients {
                    recipe_text: recipe_text.clone(),
                }
            }
            _ => {
                log::warn!("find_ingredients called in invalid state: {:?}", self);
                self.clone()
            }
        }
    }

    pub fn set_error(&mut self, error: String) -> Self {
        State::Error {
            recipe_text: self.recipe_text(),
            error,
        }
    }

    pub fn set_ingredients(&mut self, ingredients: Vec<Ingredient>) -> Self {
        State::ShoppingList {
            recipe_text: self.recipe_text(),
            ingredients,
        }
    }

    pub fn reset(&mut self) -> Self {
        State::RecipeInput {
            recipe_text: self.recipe_text(),
        }
    }
}

#[component]
pub fn View() -> impl IntoView {
    // let (state, set_state) = create_signal(State::RecipeInput {
    //     recipe_text: "".to_string(),
    // });

    let (state, set_state) = create_signal(State::ShoppingList {
        recipe_text: "".to_string(),
        ingredients: vec![
            Ingredient {
                id: Uuid::new_v4(),
                name: "Magerquark".to_string(),
                probably_at_home: Some(true),
                unit: "Gram".to_string(),
                quantity: 250,
                status: IngredientStatus::Unchecked,
            },
            Ingredient {
                id: Uuid::new_v4(),
                name: "Butter".to_string(),
                probably_at_home: Some(true),
                unit: "Gram".to_string(),
                quantity: 250,
                status: IngredientStatus::ApiSearchFailed {
                    error: "Die Anfrage an Rewe ist fehlgeschlagen".to_string(),
                },
            },
            Ingredient {
                id: Uuid::new_v4(),
                name: "Butter".to_string(),
                probably_at_home: Some(true),
                unit: "Gram".to_string(),
                quantity: 250,
                status: IngredientStatus::NoSearchResults,
            },
            Ingredient {
                id: Uuid::new_v4(),
                name: "Butter".to_string(),
                probably_at_home: Some(true),
                unit: "Gram".to_string(),
                quantity: 250,
                status: IngredientStatus::SearchResults {
                    items: vec![Item {
                        id: Uuid::new_v4(),
                        name: "Kerrygold".to_string(),
                        quantity: Some("250 g".to_string()),
                        price_cent: Some(150),
                        url: Some("https://www.rewe.de/produkte/1234".to_string()),
                        image_url: Some("https://www.rewe.de/produkte/1234/image.jpg".to_string()),
                    }],
                },
            },
            Ingredient {
                id: Uuid::new_v4(),
                name: "Butter".to_string(),
                probably_at_home: Some(true),
                unit: "Gram".to_string(),
                quantity: 250,
                status: IngredientStatus::AiFailsToSelectItem {
                    alternatives: vec![Item {
                        id: Uuid::new_v4(),
                        name: "Kerrygold".to_string(),
                        quantity: Some("250 g".to_string()),
                        price_cent: Some(150),
                        url: Some("https://www.rewe.de/produkte/1234".to_string()),
                        image_url: Some("https://www.rewe.de/produkte/1234/image.jpg".to_string()),
                    }],
                },
            },
            Ingredient {
                id: Uuid::new_v4(),
                name: "Butter".to_string(),
                probably_at_home: Some(true),
                unit: "Gram".to_string(),
                quantity: 250,
                status: IngredientStatus::Matched {
                    item: Item {
                        id: Uuid::new_v4(),
                        name: "Kerrygold".to_string(),
                        quantity: Some("250 g".to_string()),
                        price_cent: Some(150),
                        url: Some("https://www.rewe.de/produkte/1234".to_string()),
                        image_url: Some("https://jflessau.com/img/placeholder.jpg".to_string()),
                    },
                    pieces: 1,
                    alternatives: vec![
                        Item {
                            id: Uuid::new_v4(),
                            name: "Kerrygold".to_string(),
                            quantity: Some("250 g".to_string()),
                            price_cent: Some(350),
                            url: Some("https://www.rewe.de/produkte/1234".to_string()),
                            image_url: Some(
                                "https://www.rewe.de/produkte/1234/image.jpg".to_string(),
                            ),
                        },
                        Item {
                            id: Uuid::new_v4(),
                            name: "Kerrygold ungesalzen".to_string(),
                            quantity: Some("250 g".to_string()),
                            price_cent: Some(450),
                            url: Some("https://www.rewe.de/produkte/1234".to_string()),
                            image_url: Some(
                                "https://www.rewe.de/produkte/1234/image.jpg".to_string(),
                            ),
                        },
                        Item {
                            id: Uuid::new_v4(),
                            name: "Markenbutter".to_string(),
                            quantity: Some("250 g".to_string()),
                            price_cent: Some(550),
                            url: Some("https://www.rewe.de/produkte/1234".to_string()),
                            image_url: Some(
                                "https://www.rewe.de/produkte/1234/image.jpg".to_string(),
                            ),
                        },
                    ],
                },
            },
        ],
    });

    view! {
        <div class="w-full flex flex-col items-center justify-center gap-6">
            <img
                class="w-32 h-32"
                src="/img/logo.png"
                alt="shopping bag with vegetables, fruits and beverages"
            />
            // <h1 class="mb-6 text-m">"koch-doch-einfach.org"</h1>

            <div class="w-full flex flex-col items-center justify-start gap-12">
                {move || {
                    match state() {
                        State::Error { error, .. } => {
                            view! {
                                <p class="w-full text-center font-bold text-error">
                                    {error.clone()}
                                </p>

                                <button
                                    on:click=move |_| set_state(State::RecipeInput {
                                        recipe_text: state().recipe_text(),
                                    })
                                    class="px-2 flex gap-1 items-center text-info text-bold text-s border border-info rounded"
                                >
                                    <Icon icon=i::TbArrowLoopLeft2 width="0.9rem" height="0.9rem" />
                                    "Nochmal versuchen"
                                </button>
                            }
                                .into_view()
                        }
                        State::RecipeInput { recipe_text } => {
                            view! { <RecipeInput set_state recipe_text /> }.into_view()
                        }
                        State::FindIngredients { .. } => {
                            view! {
                                <LoadingIndicator
                                    title="Ermittle Zutaten...".to_string()
                                    subtitle="Ich geb mir große Mühe!".to_string()
                                />
                            }
                                .into_view()
                        }
                        State::ShoppingList { ingredients, .. } => {
                            view! {
                                <button
                                    on:click=move |_| set_state(state().reset())
                                    class="px-2 flex gap-1 items-center text-info text-bold text-s border border-info rounded"
                                >
                                    <Icon icon=i::LuFileEdit width="0.9rem" height="0.9rem" />
                                    "Rezept ändern"
                                </button>

                                <ShoppingList ingredients />
                            }
                                .into_view()
                        }
                    }
                }}
            </div>
        </div>
    }
}
