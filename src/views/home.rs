use crate::components::{
    find_ingredients_button::FindIngredientsButton, loading_indicator::LoadingIndicator,
    recipe_input::View as RecipeInput,
};
use crate::prelude::*;

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
        ingredients: Vec<String>,
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

    pub fn ingredients(&self) -> Option<&Vec<String>> {
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

    pub fn set_ingredients(&mut self, ingredients: Vec<String>) -> Self {
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
    let (state, set_state) = create_signal(State::RecipeInput {
        recipe_text: "".to_string(),
    });

    create_effect(move |_| {
        log::info!("state: {:?}", state());
    });

    view! {
        <div class="w-full flex flex-col items-center justify-center gap-6">
            <img
                class="w-32 h-32"
                src="/img/logo.png"
                alt="shopping bag with vegetables, fruits and beverages"
            />

            <h1 class="mb-8 text-m">"koch-doch-einfach.org"</h1>

            <div class="w-full flex flex-col items-center justify-start gap-6">
                {
                    move || {
                        match state() {
                            State::Error { error, .. } => {
                                view! {
                                    <p class="w-full text-center font-bold text-error">
                                        {error.clone()}
                                    </p>
                                    <button
                                        on:click=move |_| set_state(state().reset())
                                        class="w-fit py-1 px-4 font-bold text-color-inverted bg-info rounded-lg">
                                        "Nochmal versuchen"
                                    </button>
                                }.into_view()
                            },
                            State::RecipeInput { recipe_text } => {
                                view! {
                                    <RecipeInput recipe_text set_state/>
                                    <FindIngredientsButton state set_state/>
                                }.into_view()
                            },
                            State::FindIngredients { .. } => {
                                view! {
                                    <LoadingIndicator
                                        title="Ermittle Zutaten...".to_string()
                                        subtitle="Ich geb mir große Mühe!".to_string() />
                                }.into_view()
                            },
                            State::ShoppingList { ingredients, .. } => {
                                view! {
                                    <div class="w-full flex flex-col items-center justify-start gap-6">
                                        <h2 class="text-xl text-center font-black text-attention">"Zutaten"</h2>
                                        {
                                            ingredients.iter().map(|ingredient| {
                                                view! {
                                                    <p class="text-l">{ingredient}</p>
                                                }
                                            }).collect::<Vec<_>>()
                                        }
                                    </div>
                                }.into_view()
                            },
                        }
                    }
                }
            </div>
        </div>
    }
}
