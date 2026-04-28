use std::collections::HashMap;

use dioxus::prelude::*;

use crate::home_api::{self, Image};

#[component]
pub fn Home() -> Element {
    let images = use_loader(home_api::get_images)?;
    let grouped = use_memo(move || {
        let images = images();
        let mut map: HashMap<String, Vec<Image>> = HashMap::new();
        for image in images {
            map.entry(image.time.year()).or_default().push(image);
        }
        map
    });

    rsx! {
        Content { images: grouped }
    }
}

#[component]
fn Content(images: ReadSignal<HashMap<String, Vec<Image>>>) -> Element {
    let map = images();

    rsx! {
        for year in map.keys() {
            div { class: "flex flex-col",
                p { class: "text-xl", "{year}" }
                div { class: "grid auto-cols-max grid-flow-col gap-3",
                    for image in map.get(year).expect("has key") {
                        img {
                            class: "w-auto max-h-128",
                            src: format!("data:image/png;base64, {}", image.base64),
                        }
                    }
                }
            }
        }
    }
}
