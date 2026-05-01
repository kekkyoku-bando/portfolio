use std::cmp::Reverse;
use std::collections::HashMap;

use dioxus::prelude::*;

use crate::gallery_images::{self, Image};

#[component]
pub fn Gallery() -> Element {
    let grouped = use_memo(move || {
        let mut map: HashMap<Option<u32>, Vec<Image>> = HashMap::new();
        for image in gallery_images::IMAGES.clone() {
            map.entry(image.time.year()).or_default().push(image);
        }
        map
    });

    rsx! {
        Content { images: grouped }
    }
}

#[component]
fn Content(images: ReadSignal<HashMap<Option<u32>, Vec<Image>>>) -> Element {
    const COLUMNS: usize = 4;

    let sorted: Memo<Vec<(String, Vec<Vec<Image>>)>> = use_memo(move || {
        let grouped_by_years = images();
        let mut sorted_columns: HashMap<Option<u32>, Vec<Vec<Image>>> = HashMap::new();

        for (year, mut images) in grouped_by_years {
            images.sort_by_key(|image| Reverse(image.time));

            let cols = sorted_columns
                .entry(year)
                .or_insert_with(|| vec![vec![]; COLUMNS]);
            for (i, image) in images.into_iter().enumerate() {
                cols[i % COLUMNS].push(image);
            }
        }

        let mut sorted_years: Vec<(Option<u32>, Vec<Vec<Image>>)> =
            sorted_columns.into_iter().collect();
        sorted_years.sort_by_key(|(year, _)| Reverse(year.unwrap_or(u32::MIN)));
        sorted_years
            .into_iter()
            .map(|(year, columns)| {
                (
                    year.as_ref()
                        .map(u32::to_string)
                        .unwrap_or("Unknown".to_string()),
                    columns,
                )
            })
            .collect()
    });
    let mut lightbox_image: Signal<Option<Asset>> = use_signal(|| None);

    rsx! {
        div { class: "flex flex-col",
            p { "This is a selected collection of my works throughout each year." }
            p { class: "text-base", "Email: allimehcla@gmail.com" }
            p { class: "text-base", "X: allimehcla" }
            br {}

            for (year, columns) in sorted() {
                div { class: "flex flex-col mb-2",
                    p { class: "text-4xl mb-4", {year} }
                    div { class: "grid grid-cols-4 gap-4",
                        for column in columns {
                            div { class: "grid gap-4",
                                for image in column {
                                    div {
                                        img {
                                            class: "h-auto max-w-full cursor-pointer",
                                            src: image.asset,
                                            onclick: move |_| {
                                                lightbox_image.set(Some(image.asset));
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if let Some(asset) = lightbox_image() {
            div {
                class: "fixed inset-0 z-50 flex items-center justify-center p-2 bg-black/80 cursor-pointer",
                onclick: move |_| lightbox_image.set(None),
                img {
                    class: "max-w-full max-h-[calc(100vh-4rem)] object-contain",
                    src: asset,
                }
            }
        }
    }
}
