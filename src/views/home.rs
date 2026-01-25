use dioxus::prelude::*;

const CSS: Asset = asset!("/assets/styling/home.css");

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        div { id: "hero",
            "Hero!"
        }
    }
}
