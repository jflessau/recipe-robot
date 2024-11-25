use crate::error_template::{AppError, ErrorTemplate};
use crate::prelude::*;
use crate::views::home;

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
        <meta name="viewport" content="width=device-width, initial-scale=1" />

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors /> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=home::View />
                </Routes>
            </main>
        </Router>
    }
}
