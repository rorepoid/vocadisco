use crate::Route;
use dioxus::prelude::*;

const CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: CSS }

        div {
            id: "navbar",
            Link {
                id: "logo",
                to: Route::Home {},
                img {
                    src: asset!("/assets/logo.png"),
                    alt: "logo",
                    "VocaDisco"
                },
            }
            Link {
                class: "new-channel-button",
                to: Route::Channels,
                "Crear canal"
            }
        }
    }
}
