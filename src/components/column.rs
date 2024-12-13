use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use crate::mods::{StateModel, CardModel};
use crate::components::Card;

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
                        svg { 
                            stroke_width: "2",
                            stroke: "currentColor",
                            fill: "none",
                            width: "20",
                            height: "20",
                            view_box: "0 0 24 24",
                            stroke_linecap: "round",
                            line {  
                                // <line x1="5" y1="12" x2="19" y2="12"></line>
                                x1: "5", y1: "12", x2: "19", y2: "12"
                            }
                            line {  
                                // <line x1="12" y1="5" x2="12" y2="19"></line>
                                x1: "12", y1: "5", x2: "12", y2: "19"
                            }
                        }
                    }
                    button {
                        class: "text-slate-400 hover:text-[#413a46] duration-200",
                        onclick: move |_| {
                            info!("Edit column settings");
                        },
                        svg {
                            stroke_width: "2",
                            stroke: "currentColor",
                            fill: "none",
                            width: "20",
                            height: "20",
                            view_box: "0 0 24 24",
                            stroke_linecap: "round",
                            // <circle cx="12" cy="12" r="1"></circle><circle cx="12" cy="5" r="1"></circle><circle cx="12" cy="19" r="1"></circle>
                            circle { cx: "12", cy: "12", r: "1" }
                            circle { cx: "12", cy: "5", r: "1" }
                            circle { cx: "12", cy: "19", r: "1" }
                        }
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
