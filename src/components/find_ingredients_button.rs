use crate::prelude::*;
use crate::views::home::State;

#[component]
pub fn FindIngredientsButton(
    state: ReadSignal<State>,
    set_state: WriteSignal<State>,
) -> impl IntoView {
    view! {
        <div class="w-full flex flex-col justify-start items-center gap-6">
            <button
                on:click={
                    move |_|{
                        set_state(state().find_ingredients());
                    }
                }
                class="fancy flex items-center">
                Finde die Zutaten für mich bei Rewe!
            </button>
        </div>
    }
}
