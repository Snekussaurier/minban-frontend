use dioxus::prelude::*;

#[component]
pub fn Login(on_login: EventHandler<(String, String)>) -> Element {
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());

    rsx! {
        form {
            class: "flex flex-col justify-center gap-8 w-96 bg-white p-8 rounded-md",
            onsubmit: move |_| {
                on_login.call((username.read().clone(), password.read().clone()));
            },
            h1 { 
                class: "text-3xl text-minban_dark text-center",
                "Minban" 
            }
            input {
                class: "rounded-md p-2 bg-gray-100",
                r#type: "text",
                placeholder: "Username",
                value: "{username}",
                oninput: move |evt| username.set(evt.value())
            },
            input {
                class: "rounded-md p-2 bg-gray-100",
                r#type: "password",
                placeholder: "Password", 
                value: "{password}",
                oninput: move |evt| password.set(evt.value())
            },
            input {
                class: "rounded-md p-2 cursor-pointer bg-minban_dark text-white",
                r#type: "submit",
                value: "Login"
            }
        }
    }
}