use std::collections::HashMap;

use dioxus::prelude::*;

use crate::home_api::{self, Image};

#[component]
pub fn Home() -> Element {
    let images = use_loader(home_api::get_images)?;
    let grouped = use_memo(move || {
        let images = images();
        let mut map: HashMap<Option<u32>, Vec<Image>> = HashMap::new();
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
fn Content(images: ReadSignal<HashMap<Option<u32>, Vec<Image>>>) -> Element {
    const COLUMNS: usize = 4;

    let sorted: Memo<Vec<(String, Vec<Vec<Image>>)>> = use_memo(move || {
        let grouped_by_years = images();
        let mut sorted_columns: HashMap<Option<u32>, Vec<Vec<Image>>> = HashMap::new();

        for (year, mut images) in grouped_by_years {
            images.sort_by_key(|image| image.time);

            let cols = sorted_columns
                .entry(year)
                .or_insert_with(|| vec![vec![]; COLUMNS]);
            for (i, image) in images.into_iter().enumerate() {
                cols[i % COLUMNS].push(image);
            }
        }

        let mut sorted_years: Vec<(Option<u32>, Vec<Vec<Image>>)> =
            sorted_columns.into_iter().collect();
        sorted_years.sort_by(|(first, _), (second, _)| {
            second.unwrap_or(u32::MIN).cmp(&first.unwrap_or(u32::MIN))
        });
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

    rsx! {
        for (year, columns) in sorted() {
            div { class: "flex flex-col",
                p { class: "text-xl mb-4", {year} }
                div { class: "grid grid-cols-4 gap-4",
                    for column in columns {
                        div { class: "grid gap-4",
                            for image in column {
                                div {
                                    img {
                                        class: "h-auto max-w-full",
                                        src: format!("data:image/png;base64, {}", image.base64),
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
