use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        header {
            class: "w-full flex flex-col mt-5 px-6",
            h1 { 
                class: "text-3xl text-[#413a46]",
                "Minban" 
            }
            p { 
                class: "font-light text-[#7a6f83] text-sm mt-3",
                "A little description of the app." 
            }
        }
    }
}