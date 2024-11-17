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
pub fn View() -> impl IntoView {
    let (state, set_state) = create_signal(State::Success);

    view! {
        <div class="w-full flex flex-col justify-start items-center gap-6">
            {
                move || {
                    if !matches!(state.get(), State::Success) {
                        view! {
                            <h2 class="text-xl text-center font-black text-attention">"2. Zutaten bei Rewe finden!"</h2>
                        }.into_view()
                    } else {
                        ().into_view()
                    }
                }
            }

            {
                move || {
                    match state.get() {
                        State::Init => view! {
                            <button
                                on:click={move |_| set_state(State::Loading)}
                                class="fancy">
                                Finde die Zutaten für mich!
                            </button>
                        }.into_view(),
                        State::Loading => view! {
                            <div class="flex flex-col justify-start items-center gap-4">
                                <p class="text-attention text-l font-black" on:click={move |_| set_state(State::Error("That did not work...".to_string()))}>
                                    "Ermittle Zutaten..."
                                </p>
                                <p class="text-attention text-s font-black">
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
                            }
                        }.into_view(),
                        State::Success => shopping_list::View().into_view(),
                    }
                }
            }
        </div>
    }
}
