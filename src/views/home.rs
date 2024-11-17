use crate::components::{find_ingredients_button, recipe_input, vendor_selection};
use crate::prelude::*;

#[component]
pub fn View() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center gap-12">
            <div class="flex flex-col items-center justify-center gap-6">
                <img
                    class="w-32 h-32"
                    src="/img/logo.png"
                    alt="shopping bag with vegetables, fruits and beverages"
                />

                <h1 class="mb-8 text-m">"koch-doch-einfach.org"</h1>

                <div class="w-full flex flex-col items-center justify-start gap-12">
                    <recipe_input::View />
                    <vendor_selection::View />
                    <find_ingredients_button::View />
                </div>
            </div>

            <div class="w-full flex flex-col items-start justify-start gap-0">
                <p class="text-s font-thin">
                    "Ich glaube du brauchst diese Dinge von Rewe für"<br/>
                </p>
                <p class="text-xl font-black">
                    Spaghetti Carbonara
                </p>

                <div class="w-full mt-6 flex flex-col items-center justify-center gap-6">
                    <div class="w-full px-2 py-2 flex flex-col gap-2 items-center justify-start bg-mid rounded-lg border border-bg">
                        <div class="w-full flex justfy-start items-stretch gap-2">

                            <div class="flex flex-col items-center justify-center gap-2 h-24">
                                <Icon icon=i::BiPlusCircleRegular class="text-xl clickable"/>
                                <p class="w-10 bg-bg rounded h-8">3<span class="text-xs">"x"</span></p>
                                <Icon icon=i::BiMinusCircleRegular class="text-xl clickable"/>

                            </div>

                            <img src="/img/spaghetti.jpg" alt="Spaghetti" class="h-24 rounded-lg" />

                            <div class="w-full flex flex-col justify-between items-start">
                                <div class="w-full flex flex-col justify-start items-start">
                                    <div class="flex justify-start items-center gap-2">
                                        <Icon icon=i::RiFileList3DocumentLine class="text-l"/>
                                        <p class="text-s">Spaghetti 300g</p>
                                    </div>
                                    <div class="flex justify-start items-center gap-2">
                                        <Icon icon=i::BsCart4 class="text-attention text-l"/>
                                        <p class="font-black text-attention text-l">Birkel Spaghetti 500g</p>
                                    </div>
                                </div>
                            </div>

                            <div class="flex flex-col justify-between items-end min-w-24">
                                <p class="font-bold text-info text-l">"1."<sup>"99"</sup>" €"</p>
                                <button class="text-l text-error w-fit">
                                     <Icon icon=i::FaTrashCanRegular />
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
