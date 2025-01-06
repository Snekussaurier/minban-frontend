use std::collections::{BTreeSet, HashMap};
use dioxus::prelude::*;
use dioxus::logger::tracing::{debug, error};
use crate::mods::{StateModel, CardModel};
use crate::components::Card;
use crate::components::icons::{MoreVertical, Plus};
use crate::patch_card;
use crate::utils::{IsSelectingState, IsNewCardState};

#[component]
pub fn Column(state: StateModel, cards: BTreeSet<CardModel>) -> Element {
    let mut selected_card = use_context::<Signal<CardModel>>();
    let mut is_selecting = use_context::<Signal<IsSelectingState>>();
    let mut is_new_card = use_context::<Signal<IsNewCardState>>();
    let mut board = use_context::<Signal<HashMap<u32, BTreeSet<CardModel>>>>();

    rsx! {
        div {
            ondrop: move |_| {
                // Check if the entry for this state exists in the board and if not create it
                let board_read = board();
                let cards_option = board_read.get(&state.id);
                let cards_set = match cards_option {
                    Some(cards) => cards,
                    None => &BTreeSet::new()
                };

                // Update the position of the card
                let mut position = 1;
                if let Some(last_card) = cards_set.last() {
                    position = last_card.position + 1;
                }

                // Update the card with the new position
                let mut card = selected_card();
                card.position = position;
                card.state_id = state.id;

                let mut board = board.write();

                // Remove old card from old column
                if let Some(old_column) = board.get_mut(&selected_card().state_id) {
                    old_column.remove(&selected_card());
                };

                // Add the card to the board
                let column = board.entry(state.id).or_insert_with(BTreeSet::new);
                column.insert(card.clone());

                // Sync updated card with the database
                spawn(
                    async move {
                        let updated_card = patch_card(&card).await;
                            
                        match updated_card {
                            Ok(_) => {}
                            Err(err) => {
                                error!("Error updating card {:?}", err);
                            }
                        }
                    }
                );

                // Update the column where the card was inserted with valid positions if necessary
                let (first_card, last_card) = match (column.first(), column.last()) {
                    (Some(first_card), Some(last_card)) => (first_card, last_card),
                    _ => return
                };

                if !(first_card.position > 1 && last_card.position > 10) {return;}

                // Update positions of the remaining column in the original colum
                let mut updated_cards: Vec<CardModel> = column
                .iter()
                .map(|card| card.clone())
                .collect();

                // Reorder positions for remaining column
                for (index, card) in updated_cards.iter_mut().enumerate() {
                    card.position = (index + 1) as u32;
                }

                column.clear();
                for card in &updated_cards {
                    column.insert(card.clone());
                }

                spawn(
                    async move {
                        // patch all column that are updated
                        for card in updated_cards {
                            let updated_card = patch_card(&card).await;
                            
                            match updated_card {
                                Ok(_) => {}
                                Err(err) => {
                                    error!("Error updating card {:?}", err);
                                }
                            }
                        }
                    }
                );
            },
            ondragover: move |e| e.prevent_default(),
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
                                // Get the last card in the column and set the position to the next one
                                position: cards.len() as u32 + 1,
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
                if let Some(column) = board().get(&state.id) {
                    for card in column {Card { card: card.clone() }}
                }
            }
        }
    }
}
