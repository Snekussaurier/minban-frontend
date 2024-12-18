use dioxus::prelude::*;
use dioxus::logger::tracing::debug;
use crate::mods::{StateModel, CardModel};
use crate::components::Card;
use crate::components::icons::{MoreVertical, Plus};
use crate::utils::{IsSelectingState, IsNewCardState};

#[component]
pub fn Column(state: StateModel, cards: Vec<CardModel>) -> Element {
    let mut selected_card = use_context::<Signal<CardModel>>();
    let mut is_selecting = use_context::<Signal<IsSelectingState>>();
    let mut is_new_card = use_context::<Signal<IsNewCardState>>();

    rsx! {
        div {
            draggable: "true",
            style: "background-color: #{state.color};",
            class: "h-full w-full min-w-72 max-w-96 rounded-md p-4 flex flex-col",
            div {
                class: "flex flex-row justify-between items-center",
                p { "{state.name}" }
                div {
                    class: "flex flex-row gap-2",
                    button {
                        class: "text-slate-400 hover:text-[#413a46] duration-200",
                        onclick: move |_| {
                            // card details on with a new card in column this column
                            selected_card.set(CardModel {
                                id: "".to_string(),
                                title: "".to_string(),
                                description: "".to_string(),
                                state_id: state.id.clone(),
                                position: 1,
                                tags: vec![],
                            });
                            is_new_card.set(IsNewCardState(true));
                            is_selecting.set(IsSelectingState(true));
                        },
                        Plus {}
                    }
                    button {
                        class: "text-slate-400 hover:text-[#413a46] duration-200",
                        onclick: move |_| {
                            debug!("Edit column settings");
                        },
                        MoreVertical {}
                    }
                }

            }
            div {
                class: "flex flex-col grow overflow-y-auto mt-4 rounded-md gap-4",
                for card in cards {
                    Card { card }
                }
            }
        }
    }
}
