use crate::prelude::*;

#[component]
pub fn View(
    input: ReadSignal<String>,
    set_input: WriteSignal<String>,
    show_input: ReadSignal<bool>,
) -> impl IntoView {
    let input = move || input.get();

    move || {
        if show_input() {
            view! {
                <div class="w-full flex flex-col gap-6">
                    <h2 class="text-xl text-center font-black text-attention">"Rezept eingeben"</h2>
                    {
                        view! {
                            <textarea
                                id="recipe-input"
                                class="w-full h-48 max-h-96 rounded-lg"
                                placeholder="Text vom Rezept eingeben"
                                prop:value={move || input()}
                                on:input=move |event| {
                                    set_input(event_target_value(&event))
                                }
                            />
                        }
                    }
                </div>
            }
            .into_view()
        } else {
            ().into_view()
        }
    }
}
