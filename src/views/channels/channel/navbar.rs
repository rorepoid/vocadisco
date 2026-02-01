use dioxus::prelude::*;

use crate::router::Route;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                class: "back-button",
                to: Route::Home {},
                img {
                    src: asset!("/assets/back-icon.svg"),
                    alt: "Go back",
                },
            }
            Link {
                class: "new-channel-button",
                to: Route::Home {},
                "👱"
            }
        }
    }
}
