use crate::api::login;
use crate::LoginState;
use dioxus::prelude::*;
use reqwest::Error;

#[component]
pub fn Login(mut login_state: Resource<Result<LoginState, Error>>) -> Element {
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut error_text = use_signal(|| "".to_string());

    rsx! {
        form {
            class: "flex flex-col justify-center w-96 bg-white p-8 rounded-md",
            onsubmit: move |_| {
                spawn(async move {
                    match login(username.read().clone(), password.read().clone()).await {
                        Ok(_) => {
                            login_state.restart();
                        },
                        Err(_) => {
                            error_text.set("Invalid username or password.".to_string());
                        }
                    }
                });

            },
            h1 {
                class: "text-3xl text-minban_dark text-center",
                "Minban"
            }
            input {
                class: "rounded-md p-2 bg-gray-100 mt-8",
                r#type: "text",
                placeholder: "Username",
                value: "{username}",
                oninput: move |evt| username.set(evt.value())
            },
            input {
                class: "rounded-md p-2 bg-gray-100 mt-4",
                r#type: "password",
                placeholder: "Password",
                value: "{password}",
                oninput: move |evt| password.set(evt.value())
            },
            p {
                class: "text-sm text-red-400 h-6 mt-2",
                "{error_text}"
            }
            input {
                class: "rounded-md p-2 cursor-pointer bg-minban_dark hover:bg-minban_highlight transition-color duration-200 text-white mt-2",
                r#type: "submit",
                value: "Login"
            }
        }
    }
}
