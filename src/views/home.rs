use crate::components::{find_ingredients_button, recipe_input::View as RecipeInputView};
use crate::prelude::*;

#[component]
pub fn View() -> impl IntoView {
    let (input, set_input) = create_signal("".to_string());
    let (show_input, set_show_input) = create_signal(true);

    view! {
        <div class="w-full flex flex-col items-center justify-center gap-6">
            <img
                class="w-32 h-32"
                src="/img/logo.png"
                alt="shopping bag with vegetables, fruits and beverages"
            />

            <h1 class="mb-8 text-m">"koch-doch-einfach.org"</h1>

            <div class="w-full flex flex-col items-center justify-start gap-6">
                <RecipeInputView input=input set_input show_input />
                <find_ingredients_button::View set_show_input />
            </div>
        </div>
    }
}
