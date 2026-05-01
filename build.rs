use std::env;
use std::fs;
use std::path::Path;

use proc_macro2::TokenStream;
use quote::quote;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut entries: Vec<_> = fs::read_dir("assets/images")?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry
                .path()
                .extension()
                .map(|ext| ext.to_str() == Some("json"))
                .unwrap_or_default()
            {
                Some(entry)
            } else {
                None
            }
        })
        .collect();
    entries.sort_by_key(|e| e.file_name());

    let images: Vec<TokenStream> = entries
        .into_iter()
        .map(|entry| {
            let content = fs::read_to_string(entry.path()).unwrap();
            let json: Value = serde_json::from_str(&content).unwrap();

            let asset = json["asset"].as_str().expect("asset field missing in JSON");
            let time = json
                .get("time")
                .map(|time| {
                    let day = time.get("day").and_then(|v| v.as_u64()).map(|v| v as u32);
                    let month = time.get("month").and_then(|v| v.as_u64()).map(|v| v as u32);
                    let year = time.get("year").and_then(|v| v.as_u64()).map(|v| v as u32);

                    quote! {
                        ImageTime::Specific {
                            day: #day,
                            year: #year,
                            month: #month
                        }
                    }
                })
                .unwrap_or(quote! { ImageTime::Unknown });
            quote! {
                Image {
                    asset: asset!(#asset),
                    time: #time
                }
            }
        })
        .collect();
    let definitions = quote! {
        use std::sync::LazyLock;
        use std::cmp::Ordering;
        use dioxus::prelude::*;

        pub static IMAGES: LazyLock<Vec<Image>> = LazyLock::new(|| { vec![#(#images),*] });

        #[derive(PartialEq, Clone)]
        pub struct Image {
            pub asset: Asset,
            pub time: ImageTime,
        }

        #[derive(PartialEq, Eq, Clone, Copy)]
        pub enum ImageTime {
            #[allow(dead_code)]
            Unknown,
            Specific {
                day: u32,
                month: u32,
                year: u32,
            },
        }

        impl ImageTime {
            pub fn year(&self) -> Option<u32> {
                match self {
                    ImageTime::Unknown => None,
                    ImageTime::Specific { year, .. } => Some(*year),
                }
            }
        }

        impl PartialOrd for ImageTime {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for ImageTime {
            fn cmp(&self, other: &Self) -> Ordering {
                match (self, other) {
                    (ImageTime::Unknown, ImageTime::Unknown) => Ordering::Equal,
                    (ImageTime::Unknown, ImageTime::Specific { .. }) => Ordering::Less,
                    (ImageTime::Specific { .. }, ImageTime::Unknown) => Ordering::Greater,
                    (
                        ImageTime::Specific {
                            year: y1,
                            month: m1,
                            day: d1,
                        },
                        ImageTime::Specific {
                            year: y2,
                            month: m2,
                            day: d2,
                        },
                    ) => y1.cmp(y2).then(m1.cmp(m2)).then(d1.cmp(d2)),
                }
            }
        }
    };

    let out_dir = env::var("OUT_DIR").unwrap();
    let output_file = Path::new(&out_dir).join("gallery_images.rs");
    let parsed = syn::parse2(definitions).unwrap();
    let formatted = prettyplease::unparse(&parsed);

    fs::write(&output_file, formatted).unwrap();
    println!("cargo:rerun-if-changed=assets/images/");

    Ok(())
}
