#![allow(non_snake_case)]

use api::*;
use std::result::Result::Ok;
use dioxus::prelude::*;
use dioxus::logger::tracing::{error, info};
use components::*;
use mods::*;
use reqwest::Error;
use utils::{IsSelectingState, IsNewCardState, LoginState};
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
    let mut cards_signal = use_context_provider(|| Signal::new(vec![] as Vec<CardModel>));
    let cards = use_resource(move || async move {
            gloo_timers::future::TimeoutFuture::new(5000).await;
            let cards_collected = get_cards().await;
            match cards_collected {
                Ok(cards) => Ok(cards_signal.set(cards)),
                Err(err) => Err(err),
            }
        }
    );
    // TODO: This can be moved inside "let cards" so that it is not called twice and all the prefetching is handled there
    let states = use_resource(move || get_states());
    let _selected_card_context = use_context_provider(|| Signal::new(CardModel::default()));
    let _is_selecting = use_context_provider(|| Signal::new(IsSelectingState(false)));
    let _is_new_card = use_context_provider(|| Signal::new(IsNewCardState(false)));

    match( &*states.read_unchecked(),&*cards.read_unchecked() ) {
        (Some(Ok(states_list)), Some(Ok(_))) => {
            rsx! {
                div{
                    class: "flex flex-col h-full w-full py-6 bg-white",
                    Header {  }
                    CardDetails {
                        on_create: move |card: CardModel| {
                            spawn(async move {
                                let created_card_id: Result<String, Error> = create_card(card.clone()).await;
                                match created_card_id {
                                    Ok(id_str) => {
                                        let mut card = card.clone();
                                        card.id = id_str;
                                        cards_signal.write().push(card);
                                    }
                                    Err(err) => {
                                        error!("Error creating card: {:?}", err);
                                    }
                                }
                            });
                        },
                        on_update: move |card: CardModel| {
                            spawn(async move {
                                let updated_card = patch_card(card.clone()).await;
                                match updated_card {
                                    Ok(_) => {
                                        let cards = cards_signal.clone();
                                        let index = cards.iter().position(|x| x.id == card.id).unwrap();
                                        cards_signal.write()[index] = card;
                                    }
                                    Err(err) => {
                                        error!("Error updating card {:?}", err);
                                    }
                                }
                            });
                        }
                    }
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
                                info!("Add state");
                            },
                            Plus {}
                        }
                    }
                }
            }
        },
        (Some(Err(states_err)), Some(Err(cards_err))) => {
            rsx! { div { "Error loading states {states_err} {cards_err}" } }
        },
        _ => {
            rsx! { Loading {} }
        }
    }
}