#![allow(non_snake_case)]

use api::*;
use reqwest::Error;
use std::collections::{BTreeSet, HashMap};
use std::result::Result::Ok;
use dioxus::prelude::*;
use dioxus::logger::tracing::{error, info};
use components::*;
use mods::*;
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
    let mut board_signal = use_context_provider(|| Signal::new(HashMap::new()));
    let board = use_resource(move || async move {
            let board_collected = get_cards().await;
            match board_collected {
                Ok(board) => Ok(board_signal.set(board)),
                Err(err) => Err(err),
            }
        }
    );
    // TODO: This can be moved inside "let cards" so that it is not called twice and all the prefetching is handled there
    let states = use_resource(move || get_states());
    let _selected_card_context = use_context_provider(|| Signal::new(CardModel::default()));
    let _is_selecting = use_context_provider(|| Signal::new(IsSelectingState(false)));
    let _is_new_card = use_context_provider(|| Signal::new(IsNewCardState(false)));

    match( &*states.read_unchecked(),&*board.read_unchecked() ) {
        (Some(Ok(states_list)), Some(Ok(_))) => {
            rsx! {
                div{
                    class: "flex flex-col h-full w-full py-6 bg-white",
                    Header {  }
                    CardDetails {
                        on_create: move |mut card: CardModel| {
                            spawn(async move {
                                let created_card_id: Result<String, Error> = create_card(&card).await;
                                match created_card_id {
                                    Ok(id_str) => {
                                        card.id = id_str;
                                        board_signal.write().entry(card.state_id).or_insert_with(BTreeSet::new).insert(card);
                                    }
                                    Err(err) => {
                                        error!("Error creating card: {:?}", err);
                                    }
                                }
                            });
                        },
                        on_update: move |card: CardModel| {
                            let mut board = board_signal.write();
                            let column = board.entry(card.state_id).or_insert_with(BTreeSet::new);
                        
                            column.replace(card.clone());
                            spawn(async move {
                                let updated_card = patch_card(&card).await;
                                
                                match updated_card {
                                    Ok(_) => {}
                                    Err(err) => {
                                        error!("Error updating card {:?}", err);
                                    }
                                }
                            });
                        },
                        on_delete: move |card: CardModel| {
                            let mut board = board_signal.write();
                            let cards = board.entry(card.state_id).or_insert_with(BTreeSet::new);
                            cards.remove(&card);

                            spawn(async move {
                                let deleted_card = delete_card(&card.id).await;
                                match deleted_card {
                                    Ok(_) => {}
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
                                cards: {
                                    let board = board_signal.read();
                                    let cards_option = board.get(&state.id);
                                    match cards_option {
                                        Some(cards) => cards.clone(),
                                        None => BTreeSet::new()
                                    }
                                },
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