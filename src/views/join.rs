use crate::api::join;
use crate::components::loading_indicator::LoadingIndicator;
use crate::prelude::*;
use leptos_router::Redirect;

#[derive(Debug, Clone)]
enum JoinState {
    Idle,
    Loading,
    Error { error: String },
    Success { password: String },
    PasswordSaved,
}

#[component]
pub fn View() -> impl IntoView {
    let (state, set_state) = create_signal(JoinState::Idle);
    let (input, set_input) = create_signal(String::new());

    view! {
        <div class="w-full flex flex-col items-center justify-center gap-12">
            <img
                class="w-32 h-32"
                src="/img/logo.png"
                alt="shopping bag with vegetables, fruits and beverages"
            />

            {move || match state() {
                JoinState::Idle => {
                    view! {
                        <form
                            class="w-full flex flex-col justify-start items-center gap-12"
                            on:submit=move |_| {
                                let password = input();
                                spawn_local(async move {
                                    set_state.set(JoinState::Loading);
                                    match join(password).await {
                                        Err(err) => {
                                            set_state
                                                .set(JoinState::Error {
                                                    error: err.to_string(),
                                                });
                                        }
                                        Ok(res) => {
                                            set_state
                                                .set(JoinState::Success {
                                                    password: res,
                                                });
                                        }
                                    }
                                });
                            }
                        >
                            <div class="w-full flex flex-col gap-1">

                                <label for="password-input" class="text-center text-s font-bold">
                                    Bitte gib deinen Einladungscode ein
                                </label>
                                <input
                                    id="invite-code-input"
                                    type="text"
                                    placeholder="Einladungscode"
                                    class="rounded-lg text-center"
                                    prop:value=input
                                    on:input=move |ev| set_input(event_target_value(&ev))
                                />

                            </div>
                            <button
                                disabled=move || {
                                    input().is_empty() || matches!(state(), JoinState::Loading)
                                }
                                class="fancy"
                            >
                                Registrieren
                            </button>
                        </form>
                    }
                        .into_view()
                }
                JoinState::Loading => {

                    view! {
                        <LoadingIndicator
                            title="Prüfe Passwort...".to_string()
                            subtitle="Ich hab's gleich".to_string()
                        />
                    }
                        .into_view()
                }
                JoinState::Error { error } => {
                    view! { <p>Fehler beim Login: {error}</p> }.into_view()
                }
                JoinState::Success { password } => {
                    view! {
                        <h1 class="leading-relaxed text-attention">"Registrierung erfolgreich!"</h1>
                        <p class="">
                            "Dein Login-Passwort lautet:" <br /><br />
                            <span class="font-bold text-l">{password}<br /><br /></span>
                            <span class="text">
                                "Das ist das einzige Mal, dass es dir angezeigt wird. "
                                "Bitte speichere es JETZT ab. Zum Beispiel in einem Passwort-Manager. "
                            </span>
                        </p>
                        <button
                            on:click=move |_| set_state.set(JoinState::PasswordSaved)
                            class="fancy"
                        >
                            Zum Login
                        </button>
                    }
                        .into_view()
                }
                JoinState::PasswordSaved => view! { <Redirect path="/login" /> }.into_view(),
            }}
        </div>
    }
}
