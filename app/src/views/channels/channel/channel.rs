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
        Track::new(
            "lS62a_r7dKo".to_string(),
            "WHAT YOU WANT!".to_string(),
            "https://i.ytimg.com/vi/lS62a_r7dKo/hqdefault.jpg".to_string(),
            vec!["asteria - Topic".to_string()],
        ),
        Track::new(
            "GYqbdR7GSio".to_string(),
            "Why Do I".to_string(),
            "https://i.ytimg.com/vi/GYqbdR7GSio/hqdefault.jpg".to_string(),
            vec!["Set It Off - Topic".to_string()],
        ),
        Track::new(
            "GC3_vIlIGYQ".to_string(),
            "赤と白と黒の系譜/Genealogy of Red, White and Black【鏡音リン・レン、Lily feat. team OS】".to_string(),
            "https://i.ytimg.com/vi/GC3_vIlIGYQ/hqdefault.jpg".to_string(),
            vec![
                "ひとしずく×やま△ （Hitoshizuku_Yamasankakkei）".to_string(),
            ],
        ),
        Track::new(
            "u5NBCGyuXA4".to_string(),
            "夕立のりぼん feat.MAYU".to_string(),
            "https://i.ytimg.com/vi/u5NBCGyuXA4/hqdefault.jpg".to_string(),
            vec![
                "MikitoP - Topic".to_string(),
            ],
        ),
        Track::new(
            "8Cm-7oCq9HA".to_string(),
            "[MV] TAK - ‘PPPP’ feat. 初音ミク、重音テト".to_string(),
            "https://i.ytimg.com/vi/8Cm-7oCq9HA/hqdefault.jpg".to_string(),
            vec![
                "TAK / DORIDORI".to_string(),
            ],
        ),
    ];

    rsx! {
        document::Link { rel: "stylesheet", href: CSS }

        Navbar {  }
        main {
            id: "channel-view",
            div { class: "insights",
                div {
                    h2 { "Conectados" }
                    span { class: "listeners", "42" }
                }
                div {
                    h2 { "Restante" }
                    span { class: "remaining", {format!("{} ", tracks.len())} }
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
