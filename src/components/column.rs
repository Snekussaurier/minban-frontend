use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use crate::mods::{StateModel, CardModel};
use crate::components::Card;
use crate::components::icons::{MoreVertical, Plus};

#[component]
pub fn Column(state: StateModel, cards: Vec<CardModel>) -> Element {
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
                            info!("Add card");
                        },
                        Plus {}
                    }
                    button {
                        class: "text-slate-400 hover:text-[#413a46] duration-200",
                        onclick: move |_| {
                            info!("Edit column settings");
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
