use crate::prelude::*;

#[component]
pub fn ResetRecipeButton(#[prop(into)] on_click: Callback<MouseEvent>) -> impl IntoView {
    view! {
        <button
            on:click=on_click
            class="px-2 flex gap-1 items-center text-info text-bold text-s border border-info rounded"
        >
            <Icon icon=i::LuFileEdit width="0.9rem" height="0.9rem"/>
            "Rezept ändern"
        </button>
    }
}
