use crate::api::login;
use crate::components::loading_indicator::LoadingIndicator;
use crate::prelude::*;
use leptos_router::Redirect;

#[derive(Debug, Clone)]
pub enum AuthState {
    Idle,
    Loading,
    Error { error: String },
    Success,
}

#[component]
pub fn View() -> impl IntoView {
    let (state, set_state) = create_signal(AuthState::Idle);
    let (username, set_username) = create_signal(String::new());
    let (pwd, set_pwd) = create_signal(String::new());

    view! {
        <div class="w-full flex flex-col items-center justify-center gap-12">
            <img
                class="w-32 h-32"
                src="/img/logo.png"
                alt="shopping bag with vegetables, fruits and beverages"
            />

            {move || match state() {
                AuthState::Idle => {
                    view! {
                        <form class="w-full flex flex-col justify-start items-center gap-12">
                            <div class="w-full flex flex-col gap-1">
                                <label for="username-input" class="text-center text-s font-bold">
                                    Nutzername
                                </label>
                                <input
                                    id="username-input"
                                    type="text"
                                    placeholder="Passwort"
                                    class="rounded-lg text-center"
                                    prop:value=username
                                    on:input=move |ev| set_username(event_target_value(&ev))
                                />

                                <label for="password-input" class="text-center text-s font-bold">
                                    Passwort
                                </label>
                                <input
                                    id="password-input"
                                    type="password"
                                    placeholder="Passwort"
                                    class="rounded-lg text-center"
                                    prop:value=pwd
                                    on:input=move |ev| set_pwd(event_target_value(&ev))
                                />
                            </div>
                            <button
                                disabled=move || {
                                    pwd().is_empty() || username().is_empty()
                                        || matches!(state(), AuthState::Loading)
                                }
                                on:click=move |_| {
                                    let username = username();
                                    let password = pwd();
                                    spawn_local(async move {
                                        set_state.set(AuthState::Loading);
                                        match login(username, password).await {
                                            Err(err) => {
                                                set_state
                                                    .set(AuthState::Error {
                                                        error: err.to_string(),
                                                    });
                                            }
                                            Ok(_) => {
                                                set_state.set(AuthState::Success);
                                            }
                                        }
                                    });
                                }
                                class="fancy"
                            >
                                Login
                            </button>
                        </form>
                    }
                        .into_view()
                }
                AuthState::Loading => {

                    view! {
                        <LoadingIndicator
                            title="Prüfe Passwort...".to_string()
                            subtitle="Ich hab's gleich".to_string()
                        />
                    }
                        .into_view()
                }
                AuthState::Error { error } => {
                    view! { <p>Fehler beim Login: {error}</p> }.into_view()
                }
                AuthState::Success => view! { <Redirect path="/" /> }.into_view(),
            }}
        </div>
    }
}
