use crate::prelude::*;
use crate::views::home::State;

#[component]
pub fn View(set_state: WriteSignal<State>, recipe_text: String) -> impl IntoView {
    let textarea_ref: NodeRef<html::Textarea> = create_node_ref();

    create_effect(move |_| {
        if let Some(ref_input) = textarea_ref.get() {
            let _r = ref_input.on_mount(|input| {
                let _r = input.focus();
            });
        }
    });

    view! {
        <div class="w-full flex flex-col gap-6">
            <h2 class="text-xl text-center font-black text-attention">"Rezept eingeben"</h2>
            {
                view! {
                    <textarea
                        id="recipe-input"
                        node_ref=textarea_ref
                        class="w-full h-48 max-h-96 rounded-lg"
                        placeholder="Text vom Rezept eingeben"
                        prop:value={recipe_text}
                        on:input=move |event| {
                            set_state(State::RecipeInput {
                                recipe_text: event_target_value(&event),
                            });
                        }
                    />
                }
            }
        </div>
    }
}
