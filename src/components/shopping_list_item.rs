use crate::prelude::*;

#[component]
pub fn View() -> impl IntoView {
    view! {
        <div class="w-full p-2 flex flex-col justfy-start items-stretch gap-2 rounded-lg bg-mid">
            <div class="w-full flex justify-start gap-2">
                <img src="/img/spaghetti.jpg" alt="Spaghetti" class="w-full object-cover h-40 rounded-lg" />

                <div class="w-10 flex flex-col justify-between gap-4">
                    <div class="w-full flex flex-col items-center justify-center bg-bg rounded-lg">
                        <Icon icon=i::BiPlusCircleRegular class="w-full h-8 py-1.5 clickable"/>
                        <p class="w-full h-8 border-y border-mid">3<span class="text-xs">"x"</span></p>
                        <Icon icon=i::BiMinusCircleRegular class="w-full h-8 py-1.5 clickable"/>
                    </div>
                    <button class="w-full p-2 flex justify-center text-l text-color-inverted bg-error rounded-lg">
                        <Icon icon=i::FaTrashCanRegular />
                    </button>
                </div>
            </div>

            <div class="w-full flex flex-col justify-between items-start">
                <div class="w-full flex flex-col justify-start items-start">
                    <div class="flex justify-start items-center gap-2">
                        <Icon icon=i::RiFileList3DocumentLine class="text-l text-center w-6"/>
                        <p class="text-s">Spaghetti 300g</p>
                    </div>
                    <div class="flex justify-start items-center gap-2">
                        <Icon icon=i::BsCart4 class="text-attention text-l text-center w-6"/>
                        <p class="font-black text-attention text-l">Birkel Spaghetti 500g</p>
                    </div>
                    <div class="w-full flex justify-between items-center gap-2">
                        <div class="w-full flex justify-start items-center gap-2">
                            <Icon icon=i::ImCoinEuro class="text-l text-center w-5 pl-1"/>
                            <p class="font-bold">
                                "1,"
                                <sup>"99"</sup>
                            </p>
                            <p class="text-l font-bold opacity-50">
                                <span class="text-xs">
                                    "3,"
                                    <sup>"99"</sup>
                                    "/Stk."
                                </span>
                            </p>
                        </div>
                    </div>
                </div>
            </div>

        </div>
    }
}
