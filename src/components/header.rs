use dioxus::prelude::*;
use crate::components::icons::{Sun, Settings, Logout};
use crate::api::logout;
use crate::main;

#[component]
pub fn Header() -> Element {
    rsx! {
        header {
            class: "w-full flex flex-row justify-between px-6 items-start",
            div {
                class: "flex flex-col",
                h1 { 
                    class: "text-3xl text-[#413a46]",
                    "Minban" 
                }
                p { 
                    class: "font-light text-[#7a6f83] text-sm mt-3",
                    "A little description of the app." 
                }
            }
            div { 
                class: "flex flex-row gap-4",
                button {  
                    class: "text-slate-400 hover:text-[#413a46] duration-200",
                    Sun{}
                }
                button {  
                    class: "text-slate-400 hover:text-[#413a46] duration-200",
                    Settings{}
                }
                button {  
                    class: "text-slate-400 hover:text-red-400 duration-200",
                    onclick: move |_| {
                        spawn(async move {
                            // Logout
                            let _ = logout().await;
                            web_sys::window().unwrap().location().reload().unwrap();
                        });
                    },
                    Logout{}
                }
             }
        }
    }
}