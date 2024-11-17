use crate::prelude::*;

#[component]
pub fn View() -> impl IntoView {
    view! {
        <div class="flex flex-col justify-start items-center gap-6">
            <h2 class="text-xl text-center font-black text-attention">"3. Zutaten finden!"</h2>

            <div class="w-full flex gap-6 justify-center">
                <button
                    class="fancy">
                    Finde die Zutaten für mich!
                </button>
            </div>

            <div class="w-full flex flex-col items-center justify-center">
                <p class="text-attention text-l font-black">
                    "Ermittle Zutaten..."
                </p>
                <p class="text-attention text-s font-black">
                    "Ich geb mir große Mühe!"
                </p>
                <img
                    class="w-20 h-20 mt-8"
                    src="/img/loading-animation.gif"
                    alt="loading animation consisting of a a few rotatin circles"
                />
            </div>
        </div>
    }
}
