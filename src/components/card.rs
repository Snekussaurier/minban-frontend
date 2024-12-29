use dioxus::prelude::*;

use crate::mods::CardModel;
use crate::components::Tag;
use crate::utils::{IsNewCardState, IsSelectingState};

#[component]
pub fn Card(card: CardModel) -> Element {
    let mut selected_card = use_context::<Signal<CardModel>>();
    let mut is_selecting = use_context::<Signal<IsSelectingState>>();
    let mut is_new_card = use_context::<Signal<IsNewCardState>>();

    rsx! {
        div {
            draggable: "true",
            onclick: {
                let card = card.clone();
                move |_| {
                    // Open card details window
                    selected_card.set(card.clone());
                    is_new_card.set(IsNewCardState(false));
                    is_selecting.set(IsSelectingState(true));
                }
            },
            ondrag: {
                let card = card.clone();
                move |_| {
                    selected_card.set(card.clone());
                }
            },
            class: "min-h-24 w-full bg-white rounded-md p-4 shrink-0 flex flex-col gap-2 cursor-pointer relative",
            h1 { "{card.title}" }
            div {
                class: "flex flex-row flex-wrap gap-2 grow items-end",
                for tag in card.tags.iter() {
                    Tag { name: tag.name.clone(), color: tag.color.clone(), editable: false }
                }
            }
        }
    }
}