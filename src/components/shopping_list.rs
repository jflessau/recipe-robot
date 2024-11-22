use crate::components::shopping_list_item::ShoppingListItem;
use crate::{prelude::*, shopping_list::Ingredient};

#[component]
pub fn ShoppingList(ingredients: Vec<Ingredient>) -> impl IntoView {
    view! {
        <div class="w-full flex flex-col items-start justify-start gap-6">
            <div class="w-full flex flex-col justify-start items-center">
                <p class="w-full text-left text-s font-thin">
                    "Ich glaube du brauchst diese Dinge von Rewe für"<br/>
                </p>
                <p class="w-full text-left text-xl font-black">
                    Spaghetti Carbonara
                </p>
            </div>

            <div class="w-full flex flex-col gap-2 items-center justify-start">
                <For
                    each=move || ingredients.clone()
                    key=|ingredient| ingredient.clone()
                    children=move |ingredient| {
                        view! {
                            <ShoppingListItem title={ingredient.name().to_string()} />
                        }
                    }
                />
            </div>
        </div>
    }
}
