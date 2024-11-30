use crate::components::shopping_list_item::ShoppingListItem;
use crate::{prelude::*, shopping_list::Ingredient};

#[component]
pub fn ShoppingList(ingredients: Vec<Ingredient>) -> impl IntoView {
    let (ingredients, set_ingredients) = create_signal(ingredients);

    let rm_ingredient = move |id: Uuid| {
        set_ingredients.update(|ingredients| {
            ingredients.retain(|i| i.id != id);
        })
    };

    move || {
        view! {
            <div class="w-full flex flex-col items-start justify-start gap-6">
                {move || {
                    if ingredients().iter().any(|i| i.probably_at_home()) {
                        view! {
                            <div class="w-full flex flex-col justify-start items-start gap-2">
                                <p class="text-s w-full text-center leading-normal">
                                    Diese Zutaten hast du vielleicht schon zu Hause. Klicke sie an um sie zu entfernen:
                                </p>
                                <div class="w-full flex flex-row gap-2 flex-wrap justify-center">
                                    <For
                                        each=move || {
                                            ingredients().into_iter().filter(|i| i.probably_at_home())
                                        }
                                        key=|ingredient| ingredient.clone()
                                        children=move |ingredient| {
                                            let name = ingredient.name();
                                            view! {
                                                <button
                                                    on:click=move |_| {
                                                        rm_ingredient(ingredient.id);
                                                    }
                                                    class="px-2 flex gap-1 items-center text-error text-bold text-s border border-error rounded"
                                                >
                                                    <Icon icon=i::CgTrash width="0.9rem" height="0.9rem" />
                                                    {name}
                                                </button>
                                            }
                                        }
                                    />
                                </div>
                            </div>
                        }
                            .into_view()
                    } else {
                        ().into_view()
                    }
                }} <div class="w-full flex flex-col justify-start items-center gap-4">
                    <p class="mt-6 w-full text-center text-xl font-black">Deine Einkaufsliste</p>
                </div> <div class="w-full flex flex-col gap-2 items-center justify-start">
                    <For
                        each=move || ingredients()
                        key=|ingredient| ingredient.clone()
                        children=move |ingredient| {
                            view! { <ShoppingListItem ingredient /> }
                        }
                    />
                </div>
            </div>
        }
    }
}
