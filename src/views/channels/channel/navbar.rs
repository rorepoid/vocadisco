use dioxus::prelude::*;

use crate::router::Route;

const CSS: Asset = asset!("/assets/styling/channel-navbar.scss");

#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: CSS }

        nav {
            id: "channel-navbar",
            Link {
                class: "back-button",
                to: Route::Home {},
                img {
                    src: asset!("/assets/back-icon.svg"),
                    alt: "Go back",
                },
            }
            div { id: "channel-details",
                h1 { "Miku's Monday Meetup" }
                span { "Canal #402" }
            }
            Link {
                class: "add-new-people",
                to: Route::Home {},
                "👱"
            }
        }
    }
}
