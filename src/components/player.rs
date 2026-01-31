use dioxus::prelude::*;

const CSS: Asset = asset!("/assets/styling/player.css");

#[derive(Props, PartialEq, Clone)]
pub struct Track {
    id: String,
    title: String,
    thumbnail_url: String,
}

impl Track {
    pub fn new(id: String, title: String, thumbnail_url: String) -> Self {
        Self {
            id,
            title,
            thumbnail_url,
        }
    }

    fn src(&self) -> String {
        let autoplay = 1;
        let enablejsapi = 0;
        let controls = 0;
        let iv_load_policy = 3;

        format!("https://www.youtube.com/embed/{}?autoplay={}&enablejsapi={}&rel=0&controls={}&iv_load_policy={}",
            self.id,
            autoplay,
            enablejsapi,
            controls,
            iv_load_policy
        )
    }
}

#[component]
pub fn Player() -> Element {
    rsx! {
        if let Some(track) = use_context::<Option<Track>>() {
            document::Stylesheet { href: CSS }
            div {
                id: "player",
                iframe {
                    width: "100%",
                    height: "minmax(100%, 315px)",
                    src: track.src(),
                    allow: "accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share",
                    allowfullscreen: true,
                    referrerpolicy: "strict-origin-when-cross-origin"
                }
                div {
                    id: "player-controls",
                    div {
                        class: "thumbnail-wrapper",
                        img {
                            class: "thumbnail",
                            src: "{track.thumbnail_url}"
                        }
                    }
                    div {
                        class: "title",
                        span { "{track.title}" }
                    }
                    div {
                        class: "controls",
                        div {
                            class: "paused-playing-wrapper",
                            // div {
                            //     class: "paused",
                            //     svg {
                            //         xmlns: "http://www.w3.org/2000/svg",
                            //         height:"24",
                            //         width:"24",
                            //         view_box:"0 0 24 24",
                            //         style: "pointer-events: none; display: inherit; width: 100%; height: 100%;",
                            //         path {
                            //             d: "M5 4.623V19.38a1.5 1.5 0 002.26 1.29L22 12 7.26 3.33A1.5 1.5 0 005 4.623Z"
                            //         }
                            //     }
                            // }
                            div {
                                class: "playing",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    height:"24",
                                    width:"24",
                                    view_box:"0 0 24 24",
                                    style: "pointer-events: none; display: inherit; width: 100%; height: 100%;",
                                    path {
                                        d: "M6.5 3A1.5 1.5 0 005 4.5v15A1.5 1.5 0 006.5 21h2a1.5 1.5 0 001.5-1.5v-15A1.5 1.5 0 008.5 3h-2Zm9 0A1.5 1.5 0 0014 4.5v15a1.5 1.5 0 001.5 1.5h2a1.5 1.5 0 001.5-1.5v-15A1.5 1.5 0 0017.5 3h-2Z"
                                    }
                                }
                            }
                        }
                        div {
                            class: "next-track-wrapper",
                            div {
                                class: "next-track",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    height:"24",
                                    width:"24",
                                    view_box:"0 0 24 24",
                                    style: "pointer-events: none; display: inherit; width: 100%; height: 100%;",
                                    path {
                                        d: "M20 20a1 1 0 001-1V5a1 1 0 00-2 0v14a1 1 0 001 1Zm-14.955-.226L18 12 5.045 4.228A1.35 1.35 0 003 5.386v13.23a1.35 1.35 0 002.045 1.158Z"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
