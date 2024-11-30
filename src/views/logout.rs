use crate::api::logout;
use crate::components::loading_indicator::LoadingIndicator;
use crate::prelude::*;
use leptos_router::Redirect;

#[component]
pub fn View() -> impl IntoView {
    let logout = create_resource(|| (), |_| async move { logout().await.is_ok() });

    view! {
        <div class="w-full flex flex-col items-center justify-center gap-12">
            <img
                class="w-32 h-32"
                src="/img/logo.png"
                alt="shopping bag with vegetables, fruits and beverages"
            />
            <Suspense fallback=move || {
                view! {
                    <LoadingIndicator
                        title="Du wirst ausgeloggt.".to_string()
                        subtitle="Bitte warten.".to_string()
                    />
                }
            }>
                move ||
                {match logout() {
                    None => ().into_view(),
                    Some(true) => view! { <Redirect path="/login" /> }.into_view(),
                    Some(false) => {
                        view! {
                            <p class="text-center text-s font-bold">"Logout fehlgeschlagen."</p>
                        }
                            .into_view()
                    }
                }}
            </Suspense>
        </div>
    }
}
