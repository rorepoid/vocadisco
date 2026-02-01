use dioxus::prelude::*;

use crate::{
    components::{Track, TrackPlayer},
    views::channels::channel::Navbar,
};

const CSS: Asset = asset!("/assets/styling/channel.scss");

#[component]
pub fn Channel(id: u32) -> Element {
    let tracks = vec![
        Track::new(
            "y3yyYYLyVzw".to_string(),
            "【イケボでラップ、和楽器あり】初音ミク KAITO『大江戸ジュリアナイト』MV".to_string(),
            "https://i.ytimg.com/vi/y3yyYYLyVzw/hqdefault.jpg".to_string(),
            vec!["Mitchie M".to_string(), "Hatsune Miku".to_string()],
        ),
        Track::new(
            "mgeUcU0lBMA".to_string(),
            "Yo te espero (feat. 初音ミク & がくっぽいど & メグッポイド)".to_string(),
            "https://i.ytimg.com/vi/mgeUcU0lBMA/hqdefault.jpg".to_string(),
            vec![
                "AlexTrip Sands".to_string(),
                "Hatsune Miku".to_string(),
                "Gackpoid (Kamui Gakupo)".to_string(),
                "メグッポイド: Megpoid (GUMI)".to_string(),
            ],
        ),
    ];

    rsx! {
        document::Link { rel: "stylesheet", href: CSS }

        Navbar {  }
        div {
            id: "channel-view",
            div { class: "insights",
                div {
                    h2 { "Conectados" }
                    span { class: "listeners", "42" }
                }
                div {
                    h2 { "Restante" }
                    span { class: "remaining", "18 " }
                    span { "canciones" }
                }
            }
            div { class: "playlist",
                h2 { "Up Next" }
                span { "🔀 Shuffle" }
                ol {
                   for track in &tracks {
                       li { class: "item",
                            onclick: {
                                let track = track.clone();
                                move |_| consume_context::<TrackPlayer>().track.set(Some(track.clone()))
                            },
                           img {
                               src: "{track.thumbnail_url}"
                           }
                           div { class: "data",
                               h3 { class: "title", "{track.title}" }
                               span { class: "credits", "{track.inline_credits()}" }
                               span { class: "added-by", "Agregado por MarQchisan"}
                           }
                           div { class: "options",
                               "⚪️"
                           }
                       }
                   }
                }
            }
        }
    }
}
