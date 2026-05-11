//! KoloHub web shell — phased work: see `docs/frontend-phases.md`.
//! Phase 6: Dioxus Router wired; AppLayout owns theme state.

mod components;
mod pages;
mod router;
mod styles;
mod theme;

use dioxus::prelude::*;
use router::Route;
use styles::{APP_CSS, TOKENS_CSS};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Style { {TOKENS_CSS} }
        document::Style { {APP_CSS} }
        document::Title { "KoloHub — Modern, Fast, Mobile First" }
        document::Meta {
            name: "description",
            content: "KoloHub marketplace: customer app, vendor dashboard, and admin control center.",
        }
        document::Link { rel: "icon", r#type: "image/png", href: "/favicon.png" }
        Router::<Route> {}
    }
}
