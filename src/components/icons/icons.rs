use dioxus::prelude::*;

pub fn MoreVertical() -> Element {
    rsx!(
        svg {
            stroke_width: "2",
            stroke: "currentColor",
            fill: "none",
            width: "20",
            height: "20",
            view_box: "0 0 24 24",
            stroke_linecap: "round",
            // <circle cx="12" cy="12" r="1"></circle><circle cx="12" cy="5" r="1"></circle><circle cx="12" cy="19" r="1"></circle>
            circle { cx: "12", cy: "12", r: "1" }
            circle { cx: "12", cy: "5", r: "1" }
            circle { cx: "12", cy: "19", r: "1" }
        }
    )
}

pub fn Plus() -> Element {
    rsx!(                        
        svg { 
            stroke_width: "2",
            stroke: "currentColor",
            fill: "none",
            width: "20",
            height: "20",
            view_box: "0 0 24 24",
            stroke_linecap: "round",
            line {  
                // <line x1="5" y1="12" x2="19" y2="12"></line>
                x1: "5", y1: "12", x2: "19", y2: "12"
            }
            line {  
                // <line x1="12" y1="5" x2="12" y2="19"></line>
                x1: "12", y1: "5", x2: "12", y2: "19"
            }
        }
    )
}

// <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-x"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
pub fn X() -> Element {
    rsx!(
        svg {
            stroke_width: "2",
            stroke: "currentColor",
            fill: "none",
            width: "20",
            height: "20",
            view_box: "0 0 24 24",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            line {
                x1: "18", y1: "6", x2: "6", y2: "18"
            }
            line {
                x1: "6", y1: "6", x2: "18", y2: "18"
            }
        }
    )
}

// <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-trash-2"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path><line x1="10" y1="11" x2="10" y2="17"></line><line x1="14" y1="11" x2="14" y2="17"></line></svg>
pub fn TrashCan() -> Element {
    rsx!(
        svg {
            stroke_width: "2",
            stroke: "currentColor",
            fill: "none",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            polyline {
                points: "3 6 5 6 21 6"
            }
            path {
                d: "M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"
            }
            line {
                x1: "10", y1: "11", x2: "10", y2: "17"
            }
            line {
                x1: "14", y1: "11", x2: "14", y2: "17"
            }
        }
    )
}
