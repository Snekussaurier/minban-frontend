#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn Header(cx: Scope) -> Element {
    cx.render(rsx!(
        header {
            class: "w-full bg-blue-500 text-white text-center p-4",
            h1 { "Minban" }
            p { "A little description of the app." }
        }
    ))
}