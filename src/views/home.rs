use crate::components::{find_ingredients_button, recipe_input};
use crate::prelude::*;

#[component]
pub fn View() -> impl IntoView {
    view! {
        <div class="w-full flex flex-col items-center justify-center gap-12">
            <div class="flex flex-col items-center justify-center gap-6">
                <img
                    class="w-32 h-32"
                    src="/img/logo.png"
                    alt="shopping bag with vegetables, fruits and beverages"
                />

                <h1 class="mb-8 text-m">"koch-doch-einfach.org"</h1>

                <div class="w-full flex flex-col items-center justify-start gap-12">
                    <recipe_input::View />
                    <find_ingredients_button::View />
                </div>
            </div>
        </div>
    }
}
