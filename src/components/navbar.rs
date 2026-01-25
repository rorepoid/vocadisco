use crate::Route;
use dioxus::prelude::*;

const CSS: Asset = asset!("/assets/styling/navbar.css");

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This layout component wraps the UI of [Route::Home] and [Route::Blog] in a common navbar. The contents of the Home and Blog
/// routes will be rendered under the outlet inside this component
#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: CSS }

        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "VocaDisco"
            }
            Link {
                to: Route::Channels,
                "Crear nuevo canal!"
            }
        }
    }
}
