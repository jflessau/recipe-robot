use crate::{api::*, prelude::*, views::home::State};

#[component]
pub fn View(set_state: WriteSignal<State>, recipe_text: String) -> impl IntoView {
    let (text, set_text) = create_signal(recipe_text.clone());

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

        <div class="w-full flex flex-col justify-start items-center gap-6">
            <button
                on:click=move |_| {
                    let text = text.get_untracked();
                    info!("find_ingredients: recipe_text: {:?}", text);
                    spawn_local(async move {
                        match get_ingredients(text.clone()).await {
                            Err(err) => {
                                set_state.set(State::Error {
                                    recipe_text: text,
                                    error: err.to_string(),
                                });
                            },
                            Ok(ingredients) => {
                                set_state.set(State::ShoppingList {
                                    recipe_text: text,
                                    ingredients,
                                });
                            },
                        }
                    });
                }

                class="fancy flex items-center">
                Finde die Zutaten für mich bei Rewe!
            </button>
        </div>

    }
}
