use dioxus::prelude::*;

use crate::components::Navbar;
use crate::components::Player;
use crate::router::Route;

const CSS: Asset = asset!("/assets/styling/layout.css");

#[component]
pub fn Layout() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        Navbar {}
        // The `Outlet` component is used to render the next component inside the layout. In this case, it will render either
        // the [`Home`] or [`Blog`] component depending on the current route.
        Outlet::<Route> {}
        Player {}
    }
}
