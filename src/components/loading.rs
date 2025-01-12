use dioxus::prelude::*;

#[component]
pub fn Loading() -> Element {
    rsx! {
        div {
            class: "flex flex-col h-full w-full p-6 bg-white animate-pulse overflow-hidden",
            div {
                class: "w-full flex flex-col",
                div {
                    class: "h-9 w-56 bg-slate-100 rounded-full",
                }
                div {
                    class: "h-5 mt-3 flex flex-row gap-2",
                    div {
                        class: "h-full w-4 bg-slate-100 rounded-full",
                    }
                    div {
                        class: "h-full w-10 bg-slate-100 rounded-full",
                    }
                    div {
                        class: "h-full w-20 bg-slate-100 rounded-full ",
                    }
                    div {
                        class: "h-full w-6 bg-slate-100 rounded-full",
                    }
                    div {
                        class: "h-full w-8 bg-slate-100 rounded-full",
                    }
                    div {
                        class: "h-full w-10 bg-slate-100 rounded-full",
                    }
                }
            }
            // Skeleton loading
            div {
                class: "flex flex-row w-full h-full rounded-md gap-4 mt-4",
                div {
                    class: "h-full min-w-72 max-w-96 flex-grow bg-slate-100 rounded-md p-4 pt-14 flex flex-col gap-4",
                    div {
                        class: "h-24 w-full bg-white rounded-md"
                    }
                    div {
                        class: "h-24 w-full bg-white rounded-md"
                    }

                },
                div {
                    class: "h-full min-w-72 max-w-96 flex-grow bg-slate-100 rounded-md p-4 pt-14 flex flex-col gap-4",
                    div {
                        class: "h-24 w-full bg-white rounded-md"
                    }
                    div {
                        class: "h-24 w-full bg-white rounded-md"
                    }
                    div {
                        class: "h-24 w-full bg-white rounded-md"
                    }
                    div {
                        class: "h-24 w-full bg-white rounded-md"
                    }
                },
                div {
                    class: "h-full min-w-72 max-w-96 flex-grow bg-slate-100 rounded-md p-4 pt-14 flex flex-col gap-4",
                    div {
                        class: "h-24 w-full bg-white rounded-md"
                    }
                }
            }
        }
    }
}
