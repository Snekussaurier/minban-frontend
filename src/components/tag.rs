use dioxus::prelude::*;

#[component]
pub fn Tag(name: String, color: String) -> Element {
    rsx! {
        div {
            style: "background-color: #{color};",
            class: "rounded-full px-3 py-1 flex-shrink overflow-hidden relative w-fit",
            p {
                class: "text-sm",
                 "{name}"
            }
        }
    }
}
