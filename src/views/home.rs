use dioxus::prelude::*;

use crate::{components::Navbar, router::Route};

const CSS: Asset = asset!("/assets/styling/home.css");

struct Event {
    id: u32,
    title: String,
    host: String,
    participants: u32,
}

#[component]
pub fn Home() -> Element {
    let events = vec![
        Event {
            id: 1,
            title: "Miku's Monday Meetup".to_string(),
            host: "Hatsune Miku".to_string(),
            participants: 15,
        },
        Event {
            id: 2,
            title: "World is Mine".to_string(),
            host: "Hatsune Miku".to_string(),
            participants: 15,
        },
    ];

    rsx! {
        document::Stylesheet { href: CSS }
        Navbar {}
        main {
            id: "home-view",
            ul {
                for event in events {
                    Link {
                        to: Route::Channel { id: event.id },
                        li {
                            h3 { "{event.title}" }
                            div {
                                class: "host",
                                "Host: {event.host}"
                            }
                            div {
                                class: "participants",
                                "{event.participants} participantes"
                            }
                        }
                    }
                }
            }
        }
    }
}
