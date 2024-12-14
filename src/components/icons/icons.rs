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