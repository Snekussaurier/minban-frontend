use dioxus::prelude::*;

#[component]
pub fn Tag(name: String, color: String) -> Element {
    rsx! {
        div {
            style: "background-color: #{color};",
            class: "rounded-full px-3 py-1 flex-shrink overflow-hidden relative",
            p {
                class: "text-sm",
                 "{name}" 
            }
            div{
                class: "absolute top-0 left-0 w-full h-full rounded-full backdrop-blur-md hover:opacity-100 opacity-0 transition-all duration-100 bg-opacity-20 bg-white cursor-pointer flex items-center justify-center",
                "X"
            }
        }
    }
}