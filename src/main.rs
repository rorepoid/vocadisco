use crate::{components::Video, router::Route};
use dioxus::prelude::*;

mod components;
mod layout;
mod router;
mod views;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider::<Option<Video>>(|| {
        Some(Video {
            thumbnail_url: "https://i.ytimg.com/vi/y3yyYYLyVzw/hqdefault.jpg".to_string(),
            title: "【イケボでラップ、和楽器あり】初音ミク KAITO『大江戸ジュリアナイト』MV"
                .to_string(),
            id: "y3yyYYLyVzw".to_string(),
        })
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Meta { lang: "en" }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}
