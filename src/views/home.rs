use crate::api::authorized;
use crate::components::{
    loading_indicator::LoadingIndicator, recipe_input::View as RecipeInput,
    shopping_list::ShoppingList,
};
use crate::{prelude::*, shopping_list::Ingredient};
use leptos_router::Redirect;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    CheckAuth,
    Unauthorized,
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
            State::CheckAuth => "".to_string(),
            State::Unauthorized => "".to_string(),
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
    log::info!("Home::View");
    let (state, set_state) = create_signal(State::CheckAuth);
    create_local_resource(state, move |state| async move {
        if let State::CheckAuth = state {
            log::info!("checking auth");
            match authorized().await {
                Ok(authorized) => {
                    if authorized {
                        log::info!("Authorized");
                        set_state(State::RecipeInput {
                            recipe_text: "".to_string(),
                        });
                        log::info!("state set to RecipeInput");
                    } else {
                        log::warn!("Unauthorized");
                        set_state(State::Unauthorized);
                    }
                }
                Err(err) => {
                    log::error!("failed to check auth: {:?}", err);
                    set_state(State::Unauthorized);
                }
            }
        }
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
                        State::CheckAuth => {
                            view! {
                                <LoadingIndicator
                                    title="Login".to_string()
                                    subtitle="Bitte warten.".to_string()
                                />
                            }
                                .into_view()
                        }
                        State::Unauthorized => view! { <Redirect path="/login" /> }.into_view(),
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
