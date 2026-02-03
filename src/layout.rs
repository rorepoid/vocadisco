use dioxus::prelude::*;

use crate::components::Player;
use crate::router::Route;

const CSS: Asset = asset!("/assets/styling/layout.css");

#[component]
pub fn Layout() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        Outlet::<Route> {}
    }
}
