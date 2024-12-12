#![allow(non_snake_case)]

use backend_async::{checkAuth, login, Card};
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

mod backend_async;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Dashboard {},
}

#[derive(Clone, PartialEq)]
enum LoginState {
    NotLoggedIn,
    LoggedIn,
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    let mut login_state = use_resource(move || checkAuth());
    
    rsx!(
        div {
            class: "flex items-center justify-center h-screen w-screen bg-gray-100",
            match &*login_state.read_unchecked() {
                Some(Ok(LoginState::NotLoggedIn)) => rsx!( Login {
                    on_login: move |(username, password)| {
                        spawn(async move {
                            match login(username, password).await {
                                Ok(_) => {
                                    info!("Login successful");
                                    login_state.restart();
                                },
                                Err(_) => {
                                    info!("Login failed");
                                }
                            }
                        });
                    }
                }),
                Some(Ok(LoginState::LoggedIn)) => rsx!( Dashboard {} ),
                Some(Err(_)) =>  rsx!{ },
                None => rsx!(),
            }
        }
    )
}

#[component]
fn Dashboard() -> Element {
    let cards = use_resource(move || backend_async::get_cards());
    let mut cards_signal = use_signal(|| vec![]);
    let states = use_resource(move || backend_async::get_states());
    let _selected_card_context = use_context_provider(|| Signal::new(Card::default()));
    let _is_selecting = use_context_provider(|| Signal::new(false));

    rsx! {
        div {
            class: "flex flex-col h-screen w-screen py-6 bg-white",
            Header {}
            CardDetails {}
            div {
                class: "flex flex-row gap-4",
            }
            match( &*states.read_unchecked(),&*cards.read_unchecked() ) {
                (Some(Ok(states_list)), Some(Ok(cards_list))) => {
                    cards_signal.set(cards_list.clone());
                    rsx! {
                        div {
                            class: "flex flex-row w-full h-full gap-4 mt-4 overflow-x-auto overflow-y-hidden rounded-md px-6",
                            for state in states_list {
                                Column { 
                                    state: state.clone(),
                                    cards: cards_signal().iter()
                                        .filter(|card| card.state_id == state.id)
                                        .cloned()
                                        .collect::<Vec<_>>(),
                                }
                            }
                            button {
                                class: "rounded-md bg-slate-100 flex items-center justify-center h-12 w-12",
                                onclick: move |_| {
                                    info!("Add card");
                                },
                                p {
                                    class: "text-slate-500 text-lg",
                                    "+"
                                }
                            }
                        }
                    }
                },
                (Some(Err(states_err)), Some(Err(cards_err))) => {
                    rsx! { div { "Error loading states {states_err}{cards_err}" } }
                },
                _ => {
                    rsx! { div { 
                            class: "flex items-center justify-center h-full w-full",
                            "Loading..." 
                        } 
                    }
                }
            }
        }
    }
}

#[component]
fn Loading() -> Element {
    rsx! {
        div {
            class: "flex items-center justify-center h-full w-full",
            "Loading..."
        }
    }
}

#[component]
fn CardDetails() -> Element {
    let mut card = use_context::<Signal<Card>>();
    let mut is_selecting = use_context::<Signal<bool>>();

    let display = if is_selecting() { "flex".to_string() } else { "none".to_string() };

    rsx! {
        div {
            style: format!("display: {}", display),
            class: "absolute top-0 left-0 z-50 w-full h-full backdrop-blur-sm items-center justify-center bg-slate-950 bg-opacity-20",
            div {
                class: "bg-white rounded-md w-4/5 max-w-lg h-4/5 max-h-[756px] shadow-xl flex flex-col gap-4",
                div {
                    class: "p-4 bg-slate-100 rounded-t-md",
                    AutoResizeTextarea { content: card.read().title.clone() }
                    div {
                        class: "flex flex-row gap-2",
                        for tag in card.read().tags.iter() {
                            Tag { name: tag.name.clone(), color: tag.color.clone() }
                        }
                    }
                }
                div {
                    class: "flex flex-col gap-4 px-4 pb-4 h-full",
                    p {
                        class: "text-[#7a6f83] text-sm",
                        "Description"
                    }
                    textarea {
                        class: "flex-grow resize-none overflow-y-auto ",
                        placeholder: "Description",
                        value: "{card.read().description}",
                        oninput: move |evt| card.write().description = evt.value()
                    }
                    button {
                        class: "rounded-md p-2 bg-purple-100",
                        onclick: move |_| {
                            is_selecting.set(false);
                            spawn(async move {
                                let _ = backend_async::patch_card(card.read().clone()).await;
                            });
                        },
                        "Save"
                    }
                    button {
                        class: "rounded-md p-2 bg-slate-100",
                        onclick: move |_| is_selecting.set(false),
                        "Close"
                    }
                }
            }
        }
    }
}

#[component]
fn AutoResizeTextarea(content: String) -> Element {
    rsx!(
        textarea {
            class: "text-2xl text-[#413a46] w-full overflow-y-hidden bg-transparent resize-none",
            placeholder: "Title",
            maxlength: "60",
            value: "{content}"
        }
    )
}

#[component]
fn Login(on_login: EventHandler<(String, String)>) -> Element {
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());

    rsx! {
        form {
            class: "flex flex-col justify-center gap-8 w-96 bg-white p-8 rounded-md",
            prevent_default: "onclick",
            onsubmit: move |_| {
                on_login.call((username.read().clone(), password.read().clone()));
            },
            h1 { 
                class: "text-3xl text-[#413a46] text-center",
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
            button {
                class: "rounded-md p-2 bg-purple-100",
                "Login" 
            }
        }
    }
}

#[component]
fn Header() -> Element {
    rsx! {
        header {
            class: "w-full flex flex-col mt-5 px-6",
            h1 { 
                class: "text-3xl text-[#413a46]",
                "Minban" 
            }
            p { 
                class: "font-light text-[#7a6f83] text-sm mt-3",
                "A little description of the app." 
            }
        }
    }
}

#[component]
fn Column(state: backend_async::State, cards: Vec<backend_async::Card>) -> Element {
    rsx! {
        div {
            draggable: "true",
            style: "background-color: #{state.color};",
            class: "h-full w-full min-w-72 max-w-96 rounded-md p-4 flex flex-col",
            h1 { "{state.name}" }
            div {
                class: "flex flex-col grow overflow-y-auto mt-4 rounded-md gap-4",
                for card in cards {
                    CardElement { card }
                }
            }
        }
    }
}

#[component]
fn CardElement(card: Card) -> Element {
    let mut selected_card = use_context::<Signal<Card>>();
    let mut is_selecting = use_context::<Signal<bool>>();

    rsx! {
        div {
            draggable: "true",
            onclick: move |_| {
                // Open card details window
                selected_card.set(card.clone());
                is_selecting.set(true);
            },
            class: "min-h-24 w-full bg-white rounded-md p-4 shrink-0 flex flex-col gap-2 cursor-pointer relative",
            h1 { "{card.title}" }
            div {
                class: "flex flex-row flex-wrap gap-2 grow items-end",
                for tag in card.tags.iter() {
                    Tag { name: tag.name.clone(), color: tag.color.clone() }
                }
            }
        }
    }
}

#[component]
fn Tag(name: String, color: String) -> Element {
    rsx! {
        div {
            style: "background-color: #{color};",
            class: "rounded-full px-3 py-1 flex-shrink overflow-hidden relative",
            p {
                class: "text-sm",
                 "{name}" 
            }
            div{
                class: "absolute top-0 left-0 w-full h-full rounded-full backdrop-blur-md hover:opacity-100 opacity-0 transition-all duration-100 bg-opacity-20 bg-white cursor-pointer flex items-center justify-center",
                "X"
            }
        }
    }
}
