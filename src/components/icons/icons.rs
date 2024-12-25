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

// <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-log-out"><path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"></path><polyline points="16 17 21 12 16 7"></polyline><line x1="21" y1="12" x2="9" y2="12"></line></svg>
pub fn Logout() -> Element {
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
            path {
                d: "M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"
            }
            polyline {
                points: "16 17 21 12 16 7"
            }
            line {
                x1: "21", y1: "12", x2: "9", y2: "12"
            }
        }
    )
}

// <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-settings"><circle cx="12" cy="12" r="3"></circle><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path></svg>
pub fn Settings() -> Element {
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
            circle {
                cx: "12", cy: "12", r: "3"
            }
            path {
                d: "M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65
                1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"
            }
        }
    )
}

// <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-sun"><circle cx="12" cy="12" r="5"></circle><line x1="12" y1="1" x2="12" y2="3"></line><line x1="12" y1="21" x2="12" y2="23"></line><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line><line x1="1" y1="12" x2="3" y2="12"></line><line x1="21" y1="12" x2="23" y2="12"></line><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line></svg>
pub fn Sun() -> Element {
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
            circle {
                cx: "12", cy: "12", r: "5"
            }
            line {
                x1: "12", y1: "1", x2: "12", y2: "3"
            }
            line {
                x1: "12", y1: "21", x2: "12", y2: "23"
            }
            line {
                x1: "4.22", y1: "4.22", x2: "5.64", y2: "5.64"
            }
            line {
                x1: "18.36", y1: "18.36", x2: "19.78", y2: "19.78"
            }
            line {
                x1: "1", y1: "12", x2: "3", y2: "12"
            }
            line {
                x1: "21", y1: "12", x2: "23", y2: "12"
            }
            line {
                x1: "4.22", y1: "19.78", x2: "5.64", y2: "18.36"
            }
            line {
                x1: "18.36", y1: "5.64", x2: "19.78", y2: "4.22"
            }
        }
    )
}