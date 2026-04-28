use std::cmp::Ordering;
#[cfg(feature = "server")]
use std::sync::LazyLock;

#[cfg(feature = "server")]
use base64::{Engine, prelude::BASE64_STANDARD};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
static IMAGES: LazyLock<Vec<Image>> = LazyLock::new(|| {
    vec![
        Image {
            base64: BASE64_STANDARD.encode(include_bytes!("../assets/images/2026_03_21.png")),
            time: ImageTime::Specific {
                day: 21,
                month: 3,
                year: 2026,
            },
        },
        Image {
            base64: BASE64_STANDARD.encode(include_bytes!("../assets/images/2026_04_01.png")),
            time: ImageTime::Specific {
                day: 1,
                month: 4,
                year: 2026,
            },
        },
        Image {
            base64: BASE64_STANDARD.encode(include_bytes!("../assets/images/2026_04_18.png")),
            time: ImageTime::Specific {
                day: 18,
                month: 4,
                year: 2026,
            },
        },
    ]
});

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Image {
    pub base64: String,
    pub time: ImageTime,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum ImageTime {
    Unknown,
    Specific { day: u32, month: u32, year: u32 },
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

#[get("/api/images")]
pub async fn get_images() -> Result<Vec<Image>> {
    Ok(IMAGES.clone())
}
