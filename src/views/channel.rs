use dioxus::prelude::*;

#[component]
pub fn Channel(id: i32) -> Element {
    rsx! {
        div {
            "channel: {id}"
        }
    }
}
