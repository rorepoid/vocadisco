use crate::{components::TrackPlayer, router::Route};
use dioxus::prelude::*;

mod components;
mod layout;
mod router;
mod views;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.scss");
const PLAYER_CSS: Asset = asset!("/assets/styling/player.scss");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let track = use_signal(|| None);
    use_context_provider::<TrackPlayer>(|| TrackPlayer { track });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Meta { lang: "es" }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: PLAYER_CSS }

        Router::<Route> {}
    }
}
