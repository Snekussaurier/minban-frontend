use crate::components::editable_tag::EditableTag;
use crate::components::icons::{Plus, TrashCan};
use crate::components::tag::Tag;
use crate::utils::{IsNewCardState, IsSelectingState};
use crate::CardModel;
use crate::TagModel;
use dioxus::prelude::*;

#[component]
pub fn CardDetails(
    on_create: EventHandler<CardModel>,
    on_update: EventHandler<CardModel>,
    on_delete: EventHandler<CardModel>,
) -> Element {
    let mut card = use_context::<Signal<CardModel>>();

    let mut is_selecting = use_context::<Signal<IsSelectingState>>();
    let is_new_card = use_context::<Signal<IsNewCardState>>();

    let mut show_dropdown = use_signal(|| false);
    let display = if is_selecting().0 { "flex" } else { "none" };

    rsx! {
        div {
            style: "display: {display}",
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
                                id: tag.id,
                                name: tag.name.clone(),
                                color: tag.color.clone(),
                                on_click: move |id| {
                                    card.write().tags.retain(|x| x.id != id);
                                }
                            }
                        }
                                                        button {
                            onclick: move |_| { show_dropdown.set(!show_dropdown()) },
                            class: "rounded-full  px-3 py-1 flex justify-center items-center text-slate-400 hover:text-[#413a46] duration-200 relative bg-slate-200",
                            p {
                                class: "text-sm mr-1",
                                "Add tag"
                            }
                            Plus {}
                            TagSelectDropdown { card: card, is_selecting_dropdown: show_dropdown() }
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
                    div {
                        class: "flex flex-row gap-4 justify-end",
                        button {
                            class: "rounded-md p-2 bg-minban_dark text-white flex-grow duration-200",
                            onclick: move |_| {
                                if is_new_card().0 {
                                    on_create.call(card.read().clone());

                                }
                                else {
                                    on_update.call(card.read().clone());
                                }
                                is_selecting.set(IsSelectingState(false));
                                show_dropdown.set(false);
                            },
                            if is_new_card().0 {
                                "Create"
                            } else {
                                "Save"
                            }
                        }
                        button {
                            class: "rounded-md p-2 bg-slate-100 flex-grow",
                            onclick: move |_| {
                                is_selecting.set(IsSelectingState(false));
                                show_dropdown.set(false);
                            },
                            "Close"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AutoResizeTextarea(content: String) -> Element {
    rsx!(textarea {
        class: "text-2xl text-[#413a46] w-full overflow-y-hidden bg-transparent resize-none",
        placeholder: "Title",
        maxlength: "60",
        value: "{content}"
    })
}

#[component]
fn TagSelectDropdown(mut card: Signal<CardModel>, is_selecting_dropdown: bool) -> Element {
    let tags = use_context::<Signal<Vec<TagModel>>>();
    let display_value_dropdown = if is_selecting_dropdown {
        "block"
    } else {
        "none"
    };

    rsx!(
        // Add a dropdownlist for selecting adding a tag to the card
        div {
            class: "absolute top-10 right-0 w-48 bg-white rounded-md shadow-md",
            style: "display: {display_value_dropdown}",
            // Loop through tags in "tags" signal and display them as buttons
            for tag in tags() {
                if !card.read().tags.iter().any(|x| x.id == tag.id) {
                    button {
                        class: "w-full p-2 text-left hover:bg-slate-100",
                        onclick: move |_| card.write().tags.push(tag.clone()),
                        Tag {
                            name: tag.name.clone(),
                            color: tag.color.clone(),
                            editable: false
                        }
                    }
                }
            }
            hr {}
            button {
                class: "p-3 text-sm text-left hover:bg-slate-100 w-full flex flex-row gap-1",
                "Add new tag",
                Plus {}
            }
        }
    )
}
