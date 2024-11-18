use crate::prelude::*;

#[component]
pub fn LoadingIndicator(title: String, subtitle: String) -> impl IntoView {
    view! {
        <div class="flex flex-col justify-start items-center gap-4">
            <p class="text-attention text-l font-black">
                {title}
                <br></br>
                <span class="text-attention info text-s font-black">{subtitle}</span>
            </p>
            <img
                class="w-20 h-20 mt-8"
                src="/img/loading-animation.gif"
                alt="loading animation consisting of a a few rotatin circles"
            />
        </div>
    }
    .into_view()
}
