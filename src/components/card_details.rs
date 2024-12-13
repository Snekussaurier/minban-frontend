use dioxus::prelude::*;

use crate::{api::patch_card, CardModel};
use crate::components::Tag;

#[component]
pub fn CardDetails() -> Element {
    let mut card = use_context::<Signal<CardModel>>();
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
                                let _ = patch_card(card.read().clone()).await;
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
