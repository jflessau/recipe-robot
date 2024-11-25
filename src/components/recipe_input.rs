use crate::{api::*, prelude::*, views::home::State};

#[component]
pub fn View(set_state: WriteSignal<State>, recipe_text: String) -> impl IntoView {
    let (text, set_text) = create_signal(recipe_text.clone());
    let (loading, set_loading) = create_signal(false);

    view! {
        <div class="w-full flex flex-col gap-6">
            <h2 class="text-xl text-center font-black text-attention">"Rezept eingeben"</h2>
            {
                view! {
                    <textarea
                        class="w-full h-48 max-h-96 rounded-lg"
                        placeholder="Text vom Rezept eingeben"
                        on:input=move |ev| {
                            set_text(event_target_value(&ev));
                        }
                    >
                        {recipe_text.clone()}
                    </textarea>
                }
            }
        </div>

        {move || {
            if !text().is_empty() {
                view! {
                    <div class="w-full flex flex-col justify-start items-center gap-6">
                        <button
                            disabled=move || text().is_empty() || loading()
                            on:click=move |_| {
                                let text = text.get_untracked();
                                spawn_local(async move {
                                    set_loading.set(true);
                                    set_state
                                        .set(State::FindIngredients {
                                            recipe_text: text.clone(),
                                        });
                                    match get_ingredients(text.clone()).await {
                                        Err(err) => {
                                            set_loading.set(false);
                                            set_state
                                                .set(State::Error {
                                                    recipe_text: text,
                                                    error: err.to_string(),
                                                });
                                        }
                                        Ok(ingredients) => {
                                            set_loading.set(false);
                                            set_state
                                                .set(State::ShoppingList {
                                                    recipe_text: text,
                                                    ingredients,
                                                });
                                        }
                                    }
                                });
                            }

                            class="fancy flex items-center"
                        >
                            {move || {
                                if loading() {
                                    view! { "Lade..." }
                                } else {
                                    view! { "Im Supermarkt nach Zutaten suchen!" }
                                }
                            }}

                        </button>
                    </div>
                }
                    .into_view()
            } else {
                ().into_view()
            }
        }}
    }
}
