use crate::prelude::*;

#[component]
pub fn View() -> impl IntoView {
    let (recipe_input, set_recipe_input) = create_signal("".to_string());

    view! {
        <div class="flex flex-col gap-6">
            <h2 class="text-xl text-center font-black text-attention">"1. Rezept eingeben"</h2>

            <div class="flex flex-col gap-2">
                {
                    view! {
                        <textarea
                            id="recipe-input"
                            class="w-96 h-48 max-h-96 rounded-lg"
                            placeholder="Text vom Rezept eingeben"
                            prop:value=recipe_input
                            on:input=move |event| {
                                set_recipe_input(event_target_value(&event));
                            }
                        />
                    }
                }
            </div>
        </div>
    }
}
