use crate::error_template::{AppError, ErrorTemplate};
use icondata as i;
use leptos::*;
use leptos_icons::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn app() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/listoplate.css" />
        <Title text="Koch doch einfach" />
        <link rel="preconnect" href="https://fonts.googleapis.com" />
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
        <link
            href="https://fonts.googleapis.com/css2?family=EB+Garamond:ital,wght@0,400..800;1,400..800&family=Merriweather:ital,wght@0,300;0,400;0,700;0,900;1,300;1,400;1,700;1,900&family=Noto+Serif:ital,wght@0,100..900;1,100..900&display=swap"
            rel="stylesheet"
        />

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors /> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage />
                </Routes>
            </main>
        </Router>
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Vendor {
    Rewe,
    Lidl,
    Penny,
}

impl std::fmt::Display for Vendor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Vendor::Rewe => write!(f, "Rewe"),
            Vendor::Lidl => write!(f, "Lidl"),
            Vendor::Penny => write!(f, "Penny"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RecipeInput {
    Text(String),
    Link(String),
}

impl RecipeInput {
    pub fn text(&self) -> Option<&String> {
        match self {
            RecipeInput::Text(text) => Some(text),
            _ => None,
        }
    }

    pub fn link(&self) -> Option<&String> {
        match self {
            RecipeInput::Link(link) => Some(link),
            _ => None,
        }
    }

    pub fn content(&self) -> Option<String> {
        match self {
            RecipeInput::Text(text) => Some(text.clone()),
            RecipeInput::Link(link) => Some(link.clone()),
        }
    }

    pub fn similar_to(&self, other: &RecipeInput) -> bool {
        matches!(
            (self, other),
            (RecipeInput::Text(_), RecipeInput::Text(_))
                | (RecipeInput::Link(_), RecipeInput::Link(_))
        )
    }
}

impl std::fmt::Display for RecipeInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecipeInput::Text(_) => write!(f, "Text"),
            RecipeInput::Link(_) => write!(f, "Link"),
        }
    }
}

#[component]
fn home_page() -> impl IntoView {
    let (recipe_input, set_recipe_input) = create_signal(RecipeInput::Text("".to_string()));

    let select_input = move |new| {
        set_recipe_input(new);
    };

    let (vendors, set_vendors) = create_signal(vec![
        (Vendor::Rewe, false),
        (Vendor::Lidl, false),
        (Vendor::Penny, false),
    ]);

    let select_vendor = move |new| {
        let mut new_vendors = vendors.get();
        for (vendor, active) in new_vendors.iter_mut() {
            *active = *vendor == new;
        }
        set_vendors(new_vendors);
    };

    view! {
        <div class="flex flex-col items-center justify-center gap-12">
            <div class="flex flex-col items-center justify-center gap-6">
                <img
                    class="w-32 h-32"
                    src="/img/logo.png"
                    alt="shopping bag with vegetables, fruits and beverages"
                />

                <h1 class="text-m">"koch-doch-einfach.org"</h1>
            </div>

            <div class="flex flex-col gap-6">
                <h2 class="text-xl text-center font-black text-attention">"1. Rezept eingeben"</h2>

                <div class="flex flex-col gap-2">
                    <div class="w-full flex flex-row items-center gap-4">
                        <RecipeInputButton
                            caption=RecipeInput::Text("".to_string())
                            active=recipe_input
                            select_input=select_input
                        />
                        <p class="text-s min-w-20">"oder"</p>
                        <RecipeInputButton
                            caption=RecipeInput::Link("".to_string())
                            active=recipe_input
                            select_input=set_recipe_input
                        />
                    </div>

                    {
                        view! {
                            <textarea
                                id="recipe-input"
                                class="w-96 h-48 max-h-96 rounded-lg"
                                class=("hidden", move || !recipe_input().similar_to(&RecipeInput::Text("".to_string())))
                                placeholder="Text vom Rezept eingeben"
                                on:input=move |event| {
                                    set_recipe_input(RecipeInput::Text(event_target_value(&event)));
                                }
                            />
                            <input
                                id="recipe-input"
                                class="w-96 rounded-lg"
                                class=("hidden", move || !recipe_input().similar_to(&RecipeInput::Link("".to_string())))
                                placeholder="Link zum Rezept eingeben"
                                on:input=move |event| {
                                    set_recipe_input(RecipeInput::Link(event_target_value(&event)));
                                }
                            />
                        }
                    }
                </div>
            </div>

            <div class="w-full flex flex-col gap-6">
                <h2 class="text-xl text-center font-black text-attention">"2. Laden wählen"</h2>

                <div class="flex flex-row justify-center gap-4">
                    <For each=vendors key=|vendor| format!("{:?}-{}", vendor.0, vendor.1) let:item>
                        <VendorButton caption=item.0 active=item.1 select_vendor=select_vendor />
                    </For>
                </div>
            </div>

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
                                <button class="text-s underline w-fit">Alternativen anzeigen</button>
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

#[component]
pub fn RecipeInputButton(
    caption: RecipeInput,
    active: ReadSignal<RecipeInput>,
    select_input: impl Fn(RecipeInput) + 'static,
) -> impl IntoView {
    let button_classes = "w-1/2 bg-bg rounded-lg px-2 py-1 border border-contrast".to_string();
    let c = caption.clone();
    let new_input = c.clone();

    let bg = move || active().similar_to(&c);

    view! {
        <button
            class=("bg-info", bg.clone())
            class=("text-color-inverted", bg)
            on:click=move |_| {
                select_input(new_input.clone());
            }
            class=button_classes.clone()
        >
            {caption.to_string()}
        </button>
    }
}

#[component]
pub fn VendorButton(
    caption: Vendor,
    active: bool,
    select_vendor: impl Fn(Vendor) + 'static,
) -> impl IntoView {
    let button_classes = "bg-bg rounded-lg px-4 py-1 border border-contrast".to_string();
    let c = caption.clone();
    let new_vendor = c.clone();

    view! {
        <button
            class=("bg-info", move || active)
            class=("text-color-inverted", move || active)
            on:click=move |_| {
                select_vendor(new_vendor.clone());
            }
            class=button_classes.clone()
        >
            {caption.to_string()}
        </button>
    }
}
