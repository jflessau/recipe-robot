use crate::prelude::*;

#[component]
pub fn View(set_show_input: WriteSignal<bool>) -> impl IntoView {
    view! {
        <button
            on:click={move |_| set_show_input(true)}
            class="px-2 flex gap-1 items-center text-info text-bold text-s border border-info rounded"
        >
            <Icon icon=i::LuFileEdit width="0.9rem" height="0.9rem"/>
            "Rezept ändern"
        </button>
    }
}
