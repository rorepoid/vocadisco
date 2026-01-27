use dioxus::prelude::*;

#[component]
pub fn Channel(id: u32) -> Element {
    rsx! {
        div {
            "channel: {id}"
        }
    }
}
