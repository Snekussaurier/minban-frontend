use crate::components::editable_tag::EditableTag;
use crate::components::icons::{Plus, TrashCan};
use crate::components::tag::Tag;
use crate::utils::{IsNewCardState, IsSelectingState};
use crate::TagModel;
use crate::{CardModel, StateModel};
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;

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
    let color_dropdown_tag = if show_dropdown() { "#000000" } else { "" };

    rsx! {
        div {
            style: "display: {display}",
            class: "absolute top-0 left-0 z-50 w-full h-full backdrop-blur-sm items-center justify-center bg-slate-950 bg-opacity-20",
            div {
                class: "bg-white rounded-md w-4/5 max-w-lg h-4/5 max-h-[756px] shadow-xl flex flex-col gap-4",
                div {
                    style: "background-color: #{get_column_color(card().state_id)}",
                    class: "pt-3 w-full rounded-t-md",
                    div {
                        class: "p-4 bg-slate-100 rounded-t-md",
                        div {
                            class: "flex flex-row items-start gap-2",
                            AutoResizeTextarea { card: card }
                            div {
                                class: "flex flex-col justify-between",
                                if !is_new_card().0 {
                                    button {
                                        class: "h-fit text-slate-400 hover:text-red-400 duration-200 transition-colors",
                                        onclick: move |_| {
                                            on_delete.call(card.read().clone());
                                            is_selecting.set(IsSelectingState(false));
                                        },
                                        TrashCan {}
                                    }
                                }
                            }

                        }
                        div {
                            class: "flex flex-row flex-wrap gap-2 mt-4",
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
                                style: "color: {color_dropdown_tag}",
                                class: "rounded-full px-3 py-1 flex justify-center items-center text-slate-400 hover:text-black duration-200 relative bg-slate-200",
                                p {
                                    class: "text-sm mr-1",
                                    "Add tag"
                                }
                                Plus {}
                                TagSelectDropdown { card: card, is_selecting_dropdown: show_dropdown() }
                            }
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
                            class: "rounded-md p-2 bg-minban_dark hover:bg-minban_highlight text-white flex-grow duration-200",
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
                            class: "rounded-md p-2 bg-slate-100 text-slate-400 hover:text-minban_dark duration-200 flex-grow",
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
fn AutoResizeTextarea(mut card: Signal<CardModel>) -> Element {
    let mut title_area: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);

    rsx!(textarea {
        class:
            "text-2xl text-[#413a46] min-h-[33px] w-full overflow-hidden bg-transparent resize-none flex-grow",
        placeholder: "Title",
        value: "{card().title}",
        maxlength: 60,
        onmounted: move |element| {
            title_area.set(Some(element.data()));
            if let Some(element) = title_area() {
                if let Some(web_sys_element) = element.try_as_web_event() {
                    let textarea = web_sys_element.dyn_into::<HtmlTextAreaElement>().unwrap();
                    textarea.style().set_property("height", "32px").ok();
                    let scroll_height = textarea.scroll_height();
                    textarea
                        .style()
                        .set_property("height", &format!("{}px", scroll_height))
                        .ok();
                }
            }
        },
        oninput: move |evt| {
            card.write().title = evt.value();
            if let Some(element) = title_area() {
                if let Some(web_sys_element) = element.try_as_web_event() {
                    let textarea = web_sys_element.dyn_into::<HtmlTextAreaElement>().unwrap();
                    textarea.style().set_property("height", "32px").ok();
                    let scroll_height = textarea.scroll_height();
                    textarea
                        .style()
                        .set_property("height", &format!("{}px", scroll_height))
                        .ok();
                }
            }
        }
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
        div {
            class: "absolute top-10 right-0 w-48 bg-white rounded-md shadow-md",
            style: "display: {display_value_dropdown}",
            for tag in tags() {
                if !card.read().tags.iter().any(|x| x.id == tag.id) {
                    button {
                        class: "w-full p-2 text-left hover:bg-slate-100",
                        onclick: move |_| card.write().tags.push(tag.clone()),
                        Tag {
                            name: tag.name.clone(),
                            color: tag.color.clone(),
                        }
                    }
                }
            }
        }
    )
}

fn get_column_color(card_state_id: u32) -> String {
    let states_signal = use_context::<Signal<Vec<StateModel>>>();
    let states = states_signal();

    for state in states {
        if state.id == card_state_id {
            return state.color;
        };
    }

    "ffffff".to_string()
}
