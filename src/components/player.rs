use dioxus::prelude::*;

const CSS: Asset = asset!("/assets/styling/player.css");

#[component]
pub fn Player() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        div { id: "player",
            "Player!"
        }
    }
}
