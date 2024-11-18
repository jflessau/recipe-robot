use crate::components::{change_recipe_button, shopping_list_item};
use crate::prelude::*;

#[component]
pub fn View(set_show_input: WriteSignal<bool>) -> impl IntoView {
    view! {
        <div class="w-full flex flex-col items-start justify-start gap-6">
            <div class="w-full flex justify-center items-center gap-2">
                <change_recipe_button::View set_show_input />
            </div>

            <div class="w-full flex flex-col justify-start items-center">
                <p class="w-full text-left text-s font-thin">
                    "Ich glaube du brauchst diese Dinge von Rewe für"<br/>
                </p>
                <p class="w-full text-left text-xl font-black">
                    Spaghetti Carbonara
                </p>
            </div>

            <div class="w-full flex flex-col gap-2 items-center justify-start">
                <shopping_list_item::View />
            </div>
        </div>
    }
}
