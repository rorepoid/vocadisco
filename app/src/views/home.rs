use dioxus::prelude::*;
use serde::Deserialize;

use crate::{components::Navbar, router::Route};

const CSS: Asset = asset!("/assets/styling/home.css");

#[derive(Deserialize, Debug, Clone)]
struct Event {
    id: u32,
    title: String,
    host: String,
    participants: u32,
}

const MOCK_DATA_JSON: &str = include_str!("../data/events.json");

#[component]
pub fn Home() -> Element {
    let events: Vec<Event> = serde_json::from_str(MOCK_DATA_JSON).expect("No se pudo procesar el JSON");

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
