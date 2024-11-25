use crate::{prelude::*, vendor::Item};

#[component]
pub fn View<F>(select: F, item: Option<Item>, alternatives: Vec<Item>) -> impl IntoView
where
    F: Fn(Item) + Clone + Copy + 'static,
{
    let caption = if item.is_some() {
        "Alternativen"
    } else {
        "Was passt dir am besten?"
    };
    view! {
        <div class="w-full flex flex-col">
            <p class="w-full text-left text-s font-bold">{caption}</p>
            <div class="w-full flex flex-col items-start justify-start bg-bg rounded overflow-hidden">
                <For
                    each=move || { alternatives.clone() }
                    key=|alternative| alternative.id()
                    children=move |alternative| {
                        let alternative_clone = alternative.clone();
                        view! {
                            <div class="item px-2 w-full flex justify-start items-center gap-1">
                                {if item
                                    .clone()
                                    .map(|i| i.id() == alternative.id())
                                    .unwrap_or_default()
                                {
                                    view! {
                                        <Icon
                                            icon=i::BiCheckCircleSolid
                                            class="text-l"
                                            width="16px"
                                            height="16px"
                                        />
                                    }
                                        .into_view()
                                } else {
                                    ().into_view()
                                }}
                                <button
                                    on:click=move |_| {
                                        select(alternative_clone.clone());
                                    }
                                    class="w-full text-left text-s leading-normal"
                                >
                                    {alternative.name()}
                                    <span class="pl-2 text-xs text-info">
                                        {format!("{:.2} €", alternative.price())}
                                    </span>
                                </button>
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}
