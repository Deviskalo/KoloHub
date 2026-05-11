//! KoloHub web shell — phased work: see `docs/frontend-phases.md`.

use dioxus::prelude::*;
use kolohub_theme::palette::purple;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let brand = purple::P600;
    rsx! {
        div {
            "KoloHub web — Phase 1 (Dioxus + theme crate). Brand hex: {brand}"
        }
    }
}
