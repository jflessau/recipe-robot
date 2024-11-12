use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn app() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/listoplate.css" />
        <Title text="koch doch einfach" />
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

    pub fn similar_to(&self, other: &RecipeInput) -> bool {
        match (self, other) {
            (RecipeInput::Text(_), RecipeInput::Text(_)) => true,
            (RecipeInput::Link(_), RecipeInput::Link(_)) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for RecipeInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecipeInput::Text(text) => write!(f, "Text"),
            RecipeInput::Link(link) => write!(f, "Link"),
        }
    }
}

#[component]
fn home_page() -> impl IntoView {
    let (recipe_input, set_recipe_input) = create_signal(None as Option<RecipeInput>);

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
            <img
                class="w-32 h-32"
                src="/img/logo.png"
                alt="shopping bag with vegetables, fruits and beverages"
            />

            <h1>"koch-doch-einfach.org"</h1>

            <div class="flex flex-col gap-6">
                <p class="text-xl text-center font-black">"1. Rezept eingeben"</p>

                <div class="flex flex-row justify-center gap-4">
                    <RecipeInputButton
                        caption=RecipeInput::Text("".to_string())
                        active=recipe_input
                        select_input=select_input
                    />
                    <RecipeInputButton
                        caption=RecipeInput::Link("".to_string())
                        active=recipe_input
                        select_input=set_recipe_input
                    />
                </div>

                <div class="flex flex-col">
                    <label for="recipe-text-input" class="w-full text-s text-center font-thin">
                        "Text vom Rezept"
                    </label>
                    <textarea
                        id="recipe-input"
                        class="w-96 h-48 max-h-96 rounded-lg"
                        placeholder="Text vom Rezept eingeben"
                    />
                </div>

                <p>"oder"</p>

                <div class="flex flex-col">
                    <label for="recipe-link-input" class="text-s w-full text-center font-thin">
                        "Link zum Rezept"
                    </label>
                    <input
                        id="recipe-link-input"
                        class="w-96 rounded-lg text-center"
                        placeholder="Link zum Rezept eingeben"
                    />
                </div>
            </div>

            <div class="w-full flex flex-col gap-6">
                <p class="w-full text-center text-l">"2. Laden auswählen"</p>

                <div class="flex flex-row justify-center gap-4">
                    <For each=vendors key=|vendor| format!("{:?}-{}", vendor.0, vendor.1) let:item>
                        <VendorButton caption=item.0 active=item.1 select_vendor=select_vendor />
                    </For>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn RecipeInputButton(
    caption: RecipeInput,
    active: ReadSignal<Option<RecipeInput>>,
    select_input: impl Fn(Option<RecipeInput>) + 'static,
) -> impl IntoView {
    let button_classes = "bg-bg rounded px-2 py-1 border border-contrast text-s".to_string();
    let c = caption.clone();
    let new_input = c.clone();

    let bg = move || active().map(|i| i.similar_to(&c)).unwrap_or(false);

    view! {
        <button
            class=("bg-info", bg.clone())
            class=("text-fg", bg)
            on:click=move |_| {
                select_input(Some(new_input.clone()));
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
    let button_classes = "bg-bg rounded px-4 py-2 border border-contrast".to_string();
    let c = caption.clone();
    let new_vendor = c.clone();

    view! {
        <button
            class=("bg-info", move || active)
            class=("text-fg", move || active)
            on:click=move |_| {
                select_vendor(new_vendor.clone());
            }
            class=button_classes.clone()
        >
            {caption.to_string()}
        </button>
    }
}
