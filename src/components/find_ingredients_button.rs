use crate::components::shopping_list;
use crate::prelude::*;

#[derive(Debug, Clone)]
enum State {
    Init,
    Loading,
    Error(String),
    Success,
}

#[component]
pub fn View(set_show_input: WriteSignal<bool>) -> impl IntoView {
    let (state, set_state) = create_signal(State::Init);

    view! {
        <div class="w-full flex flex-col justify-start items-center gap-6">
            {
                move || match state() {
                    State::Init => view! {
                        <button
                            on:click={
                                move |_|{
                                    set_state(State::Loading);
                                    set_show_input(false);
                                }
                            }
                            class="fancy flex items-center">
                            Finde die Zutaten für mich bei Rewe!
                        </button>
                    }.into_view(),
                    State::Loading => view! {
                        <div class="flex flex-col justify-start items-center gap-4">
                            <p class="text-attention text-l font-black" on:click={move |_| set_state(State::Error("That did not work...".to_string()))}>
                                "Ermittle Zutaten..."
                            </p>
                            <p class="text-attentioninfo text-s font-black">
                                "Ich geb mir große Mühe!"
                            </p>
                            <img
                                on:click={move |_| set_state(State::Success)}
                                class="w-20 h-20 mt-8"
                                src="/img/loading-animation.gif"
                                alt="loading animation consisting of a a few rotatin circles"
                            />
                        </div>
                    }.into_view(),
                    State::Error(error) => {
                        log::info!("fails to load ingredients, error: {error}");

                        view! {
                            <p class="text-error text-l font-black">
                                "Fehler beim Ermitteln der Zutaten!"
                            </p>
                            <button
                                on:click={move |_| set_state(State::Loading)}
                                class="fancy">
                                Versuchs nochmal!
                            </button>
                        }
                    }.into_view(),
                    State::Success => {
                        view!{
                            <shopping_list::View set_show_input />
                        }
                    }
                }
            }
        </div>
    }
}
