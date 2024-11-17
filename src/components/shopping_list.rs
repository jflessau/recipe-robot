use crate::components::shopping_list_item;
use crate::prelude::*;

#[component]
pub fn View() -> impl IntoView {
    view! {
        <div class="w-full flex flex-col items-start justify-start gap-0">
            <p class="w-full text-left text-s font-thin">
                "Ich glaube du brauchst diese Dinge von Rewe für"<br/>
            </p>
            <p class="w-full text-left text-xl font-black">
                Spaghetti Carbonara
            </p>

            <div class="w-full mt-6 flex flex-col gap-2 items-center justify-start">
                {shopping_list_item::View()}
            </div>
        </div>
    }
}
