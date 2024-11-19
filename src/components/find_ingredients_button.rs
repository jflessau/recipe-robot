use crate::api::handler::get_ingredients;
use crate::views::home::State;
use leptos::*;

#[component]
pub fn FindIngredientsButton(
    state: ReadSignal<State>,
    set_state: WriteSignal<State>,
) -> impl IntoView {
    view! {
        <div class="w-full flex flex-col justify-start items-center gap-6">
            <button
                on:click=move |_| {
                    spawn_local(async move {
                        let r = get_ingredients().await;
                        log::info!("r: {:?}", r);
                    });
                }

                class="fancy flex items-center">
                Finde die Zutaten für mich bei Rewe!
            </button>
        </div>
    }
}
