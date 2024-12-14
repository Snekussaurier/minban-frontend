#![allow(non_snake_case)]

use api::*;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use components::*;
use mods::*;
use utils::LoginState;
use components::icons::Plus;

mod api;
mod components;
mod mods;
mod utils;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Dashboard {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    let mut login_state = use_resource(move || check_auth());
    
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
    let cards = use_resource(move || get_cards());
    let mut cards_signal = use_signal(|| vec![]);
    let states = use_resource(move || get_states());
    let _selected_card_context = use_context_provider(|| Signal::new(CardModel::default()));
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
                                class: "rounded-md bg-slate-100 text-slate-400 hover:text-[#413a46] duration-200 flex items-center justify-center h-12 min-w-12",
                                onclick: move |_| {
                                    info!("Add card");
                                },
                                Plus {}
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