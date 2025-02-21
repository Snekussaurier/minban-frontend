use crate::components::icons::X;
use dioxus::prelude::*;

#[component]
pub fn EditableTag(id: u32, name: String, color: String, on_click: EventHandler<u32>) -> Element {
    rsx! {
        div {
            style: "background-color: #{color};",
            class: "rounded-full px-3 py-1 flex-shrink overflow-hidden relative",
            p {
                class: "text-sm",
                 "{name}"
            }
            button {
                onclick: move |_| on_click.call(id),
                style: "background-color: #{color};",
                class: "absolute top-0 left-0 w-full h-full rounded-full backdrop-blur-md hover:opacity-100 opacity-0 transition-all duration-200 bg-opacity-20 bg-white cursor-pointer flex items-center justify-center",
                X { }
            }
        }
    }
}
