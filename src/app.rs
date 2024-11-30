use crate::error_template::{AppError, ErrorTemplate};
use crate::prelude::*;
use crate::views::{home, join, login, logout};

use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn app() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/recipe_ranger.css" />
        <Title text="Koch doch einfach" />
        <link rel="preconnect" href="https://fonts.googleapis.com" />
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
        <link
            href="https://fonts.googleapis.com/css2?family=EB+Garamond:ital,wght@0,400..800;1,400..800&family=Merriweather:ital,wght@0,300;0,400;0,700;0,900;1,300;1,400;1,700;1,900&family=Noto+Serif:ital,wght@0,100..900;1,100..900&display=swap"
            rel="stylesheet"
        />
        <meta name="viewport" content="width=device-width, initial-scale=1" />

        <link rel="apple-touch-icon" sizes="57x57" href="/img/favicon/apple-icon-57x57.png" />
        <link rel="apple-touch-icon" sizes="60x60" href="/img/favicon/apple-icon-60x60.png" />
        <link rel="apple-touch-icon" sizes="72x72" href="/img/favicon/apple-icon-72x72.png" />
        <link rel="apple-touch-icon" sizes="76x76" href="/img/favicon/apple-icon-76x76.png" />
        <link rel="apple-touch-icon" sizes="114x114" href="/img/favicon/apple-icon-114x114.png" />
        <link rel="apple-touch-icon" sizes="120x120" href="/img/favicon/apple-icon-120x120.png" />
        <link rel="apple-touch-icon" sizes="144x144" href="/img/favicon/apple-icon-144x144.png" />
        <link rel="apple-touch-icon" sizes="152x152" href="/img/favicon/apple-icon-152x152.png" />
        <link rel="apple-touch-icon" sizes="180x180" href="/img/favicon/apple-icon-180x180.png" />
        <link
            rel="icon"
            type="image/png"
            sizes="192x192"
            href="/img/favicon/android-icon-192x192.png"
        />
        <link rel="icon" type="image/png" sizes="32x32" href="/img/favicon/favicon-32x32.png" />
        <link rel="icon" type="image/png" sizes="96x96" href="/img/favicon/favicon-96x96.png" />
        <link rel="icon" type="image/png" sizes="16x16" href="/img/favicon/favicon-16x16.png" />

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors /> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=home::View />
                    <Route path="/join" view=join::View />
                    <Route path="/login" view=login::View />
                    <Route path="/logout" view=logout::View />
                </Routes>
            </main>
        </Router>
    }
}
