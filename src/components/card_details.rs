use dioxus::prelude::*;
use dioxus::logger::tracing::info;

use crate::utils::{IsNewCardState, IsSelectingState};
use crate::mods::CardModel;
use crate::components::editable_tag::EditableTag;
use crate::components::icons::{Plus, TrashCan};

#[component]
pub fn CardDetails(on_create: EventHandler<CardModel>, 
                on_update: EventHandler<CardModel>, 
                on_delete: EventHandler<CardModel>) -> Element {
    let mut card = use_context::<Signal<CardModel>>();
    let mut is_selecting = use_context::<Signal<IsSelectingState>>();
    let is_new_card = use_context::<Signal<IsNewCardState>>();

    let display = if is_selecting().0 { "flex".to_string() } else { "none".to_string() };

    rsx! {
        div {
            style: format!("display: {}", display),
            class: "absolute top-0 left-0 z-50 w-full h-full backdrop-blur-sm items-center justify-center bg-slate-950 bg-opacity-20",
            div {
                class: "bg-white rounded-md w-4/5 max-w-lg h-4/5 max-h-[756px] shadow-xl flex flex-col gap-4",
                div {
                    class: "p-4 bg-slate-100 rounded-t-md",
                    div {
                        class: "flex flex-row items-start gap-2",
                        textarea {
                            class: "text-2xl text-[#413a46] w-full overflow-y-hidden bg-transparent resize-none flex-grow",
                            placeholder: "Title",
                            maxlength: "60",
                            value: "{card.read().title}",
                            oninput: move |evt| card.write().title = evt.value()
                        }
                        if !is_new_card().0 {
                            button {
                                class: "h-fit text-slate-400 hover:text-red-400 duration-200 transition-colors",
                                onclick: move |_| {
                                    on_delete.call(card.read().clone());
                                    is_selecting.set(IsSelectingState(false));
                                },
                                TrashCan{}
                            }
                        }
                    }
                    div {
                        class: "flex flex-row gap-2",
                        for tag in card.read().tags.iter() {
                            EditableTag { 
                                id: tag.id.clone(), 
                                name: tag.name.clone(), 
                                color: tag.color.clone(), 
                                on_click: move |id| {
                                    card.write().tags.retain(|x| x.id != id);
                                }
                            }
                        }
                        button {
                            onclick: move |_| { info!("Add tag"); },
                            class: "rounded-full py-1 flex-shrink overflow-hidden relative bg-slate-200 aspect-square",
                            Plus {}
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
                            if is_new_card().0 {
                                on_create.call(card.read().clone());
                            }
                            else {
                                on_update.call(card.read().clone());
                            }
                            is_selecting.set(IsSelectingState(false));
                        },
                        if is_new_card().0 {
                            "Create"
                        } else {
                            "Save"
                        }
                    }
                    button {
                        class: "rounded-md p-2 bg-slate-100",
                        onclick: move |_| is_selecting.set(IsSelectingState(false)),
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
