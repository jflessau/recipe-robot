use crate::{
    api::*,
    components::alternative_items::View as AlternativeItems,
    prelude::*,
    shopping_list::{Ingredient, IngredientStatus},
    vendor::{Item, ReweConfig, Vendor},
};

#[component]
pub fn ShoppingListItem(ingredient: Ingredient) -> impl IntoView {
    let (ingredient, set_ingredient) = create_signal(ingredient);
    let (show_more, set_show_more) = create_signal(HashSet::<String>::new());

    create_resource(ingredient, move |ingredient| async move {
        if let IngredientStatus::Unchecked = ingredient.status() {
            let res = get_item_from_vendor(
                ingredient.clone(),
                Vendor::Rewe {
                    config: ReweConfig { zip_code: 12345 },
                },
            )
            .await;

            match res {
                Ok(ingredient) => {
                    set_ingredient(ingredient);
                }
                Err(e) => {
                    log::error!("failed to get item from vendor: {:?}", e);
                }
            }
        }
    });

    let set_pieces = move |p: usize| {
        let mut i = ingredient();
        i.set_item_pieces(p);
        set_ingredient(i);
    };

    let select_item = move |s: Item| {
        let mut i = ingredient();
        i.select_item(s.id(), None);
        set_ingredient(i);
    };

    move || {
        match ingredient().status() {
            IngredientStatus::Unchecked => view! {
                <div class="w-full p-2 flex flex-col justfy-start items-stretch gap-1 rounded-lg bg-mid">
                    <div class="flex justify-start items-center gap-2">
                        <Icon icon=i::RiFileList3DocumentLine class="icon" />
                        <p class="text-s">{move || ingredient().name()}</p>
                    </div>
                    <div class="flex justify-start items-center gap-2">
                        <Icon icon=i::BiStoreAltRegular class="icon text-attention" />
                        <p class="text-s pulsating leading-normal text-attention font-bold">
                            {"Suche im Supermarkt..."}
                        </p>
                    </div>
                </div>
            }.into_view(),
            IngredientStatus::ApiSearchFailed { .. }=> view! {
                <div class="w-full p-2 flex flex-col justfy-start items-stretch gap-1 rounded-lg bg-mid">
                    <div class="flex justify-start items-center gap-2">
                        <Icon icon=i::RiFileList3DocumentLine class="icon" />
                        <p class="text-s">{move || ingredient().name()}</p>
                    </div>
                    <div class="flex justify-start items-center gap-2">
                        <Icon icon=i::BiStoreAltRegular class="icon text-error" />
                        <p class="text-s text-error font-bold">
                            {"Die Suche im Supermarkt ist fehlgeschlagen."}
                        </p>
                    </div>
                </div>
            }.into_view(),
            IngredientStatus::NoSearchResults => view! {
                <div class="w-full p-2 flex flex-col justfy-start items-stretch gap-1 rounded-lg bg-mid">
                    <div class="flex justify-start items-center gap-2">
                        <Icon icon=i::RiFileList3DocumentLine class="icon" />
                        <p class="text-s">{move || ingredient().name()}</p>
                    </div>
                    <div class="flex justify-start items-center gap-2">
                        <Icon icon=i::BiStoreAltRegular class="icon text-error" />
                        <p class="text-s font-bold text-error">
                            {"Nichts passendes im Supermarkt gefunden."}
                        </p>
                    </div>
                </div>
            }.into_view(),
            IngredientStatus::SearchResults { items } => view! {
                <div class="w-full p-2 flex flex-col justfy-start items-stretch gap-1 rounded-lg bg-mid">
                    <div class="flex justify-start items-center gap-2">
                        <Icon icon=i::RiFileList3DocumentLine class="icon text-center" />
                        <p class="text-s">{move || ingredient().name()}</p>
                    </div>
                    <div class="flex justify-start items-center gap-2">
                        <Icon icon=i::LuFolderSearch2 class="icon text-attention" />
                        <p class="text-s pulsating text-attention font-bold">
                            {format!("Suche das passendste aus {} Produkten.", items.len())}
                        </p>
                    </div>
                </div>
            }.into_view(),
            IngredientStatus::AiFailsToSelectItem { alternatives } => view! {
                <div class="w-full p-2 flex flex-col justfy-start items-stretch gap-1 rounded-lg bg-mid">
                    <div class="flex justify-start items-center gap-2">
                        <Icon icon=i::RiFileList3DocumentLine class="icon text-center" />
                        <p class="text-s">{move || ingredient().name()}</p>
                    </div>
                    <div class="flex justify-start items-center gap-2">
                        <Icon icon=i::LuBrainCircuit class="icon text-error" />
                        <p class="text-s font-bold text-error">
                            {"Die KI konnte kein passendes Produkt finden."}
                        </p>
                    </div>
                    <AlternativeItems select=select_item item=None alternatives />
                </div>
            }.into_view(),
            IngredientStatus::Matched { item, pieces, alternatives } => {
                view! {
                    <div class="w-full p-2 flex flex-row justfy-start items-stretch gap-1 rounded-lg bg-mid">
                        <div class="w-full flex flex-col justify-between items-start">
                            <div class="w-full flex flex-col justify-start items-start">
                                <div class="w-full flex justify-start items-center gap-2">
                                    <Icon icon=i::RiFileList3DocumentLine class="text-l icon" />
                                    <p class="text-s text-left leading-normal">
                                        {move || ingredient().name()}
                                        <span class="pl-2 text-s opacity-50">
                                            {format!(
                                                "({} {})",
                                                ingredient().quantity(),
                                                ingredient().unit(),
                                            )}
                                        </span>
                                    </p>
                                </div>
                                <div class="w-full flex justify-start items-start gap-2">
                                    <Icon icon=i::BsCart4 class="text-attention icon" />
                                    <p class="font-black text-attention text-s text-left leading-normal">
                                        <a
                                            href=item.url().unwrap_or("#".to_string())
                                            target="_blank"
                                            class="text-s leading-normal"
                                        >
                                            {item.name()}
                                        </a>
                                    </p>
                                </div>
                                <div class="w-full flex justify-between items-center gap-2">
                                    <div class="w-full flex justify-start items-center gap-2">
                                        <Icon icon=i::BiCoinRegular class="text-l icon" />
                                        <p class="font-bold text-s">
                                            {format!("{:.2} €", item.price() * pieces as f32)}
                                        </p>
                                        <p class="text-s opacity-50">
                                            {format!("({:.2} € / Stück)", item.price())}
                                        </p>
                                    </div>
                                </div>
                                <button
                                    class="text-s underline"
                                    on:click=move |_| {
                                        let ingredient_name = ingredient().name();
                                        set_show_more
                                            .update(|s| {
                                                if s.contains(&ingredient_name) {
                                                    s.remove(&ingredient_name);
                                                } else {
                                                    s.insert(ingredient_name);
                                                }
                                            });
                                    }
                                >
                                    {move || {
                                        if show_more().contains(&ingredient().name()) {
                                            "weniger infos"
                                        } else {
                                            "mehr infos"
                                        }
                                    }}
                                </button>
                                {if show_more().contains(&ingredient().name()) {
                                    view! {
                                        {if let Some(image_url) = item.image_url() {
                                            view! { <img src=image_url class="w-32 h-32 rounded-lg" /> }
                                                .into_view()
                                        } else {
                                            view! {}.into_view()
                                        }}
                                        <AlternativeItems
                                            select=select_item
                                            item=Some(item.clone())
                                            alternatives
                                        />
                                    }
                                        .into_view()
                                } else {
                                    ().into_view()
                                }}
                            </div>
                        </div>
                        <div class="flex justify-start gap-2">
                            <div class="w-10 flex flex-col justify-between gap-4">
                                <div class="w-full flex flex-col items-center justify-center bg-bg rounded-lg">
                                    <button
                                        class="py-2 flex justify-center items-center clickable"
                                        on:click=move |_| {
                                            set_pieces(pieces.saturating_add(1));
                                        }
                                    >
                                        <Icon
                                            icon=i::BiPlusCircleRegular
                                            class="clickable"
                                            width="18px"
                                            height="18px"
                                        />
                                    </button>
                                    <p class="w-full h-8 border-y border-mid">
                                        {format!("{pieces}")}<span class="text-xs">"x"</span>
                                    </p>
                                    <button
                                        class="py-2 flex justify-center items-center clickable"
                                        on:click=move |_| {
                                            set_pieces(pieces.saturating_sub(1));
                                        }
                                    >

                                        <Icon
                                            icon=i::BiMinusCircleRegular
                                            width="18px"
                                            height="18px"
                                        />

                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                }.into_view()
            }
        }
    }
}
