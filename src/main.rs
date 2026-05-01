use dioxus::prelude::*;
use gallery::Gallery;

mod gallery;
mod gallery_images {
    include!(concat!(env!("OUT_DIR"), "/gallery_images.rs"));
}

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Gallery {}
    }
}
