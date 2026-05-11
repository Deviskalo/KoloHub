//! KoloHub web shell — phased work: see `docs/frontend-phases.md`.
//! Phase 2: global `tokens.css` in document head + root `kh-theme` wrapper.
//! Phase 3 (start): basic shell layout + glass nav.

use dioxus::prelude::*;
use kolohub_theme::palette::purple;

/// Shared design tokens — keep path in sync with repo layout (`design-system/tokens.css`).
const TOKENS_CSS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../design-system/tokens.css"
));

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let brand = purple::P600;
    rsx! {
        document::Style {
            {TOKENS_CSS}
        }
        div { class: "kh-theme",
            div {
                style: "position: fixed; top: 1rem; left: 50%; transform: translateX(-50%); z-index: 50; width: 100%; max-width: 42rem; padding: 0 1rem;",
                nav { class: "kh-surface-glass",
                    style: "display: flex; align-items: center; justify-content: space-between; border-radius: var(--radius-full); padding: 0.5rem 1rem;",
                    a {
                        href: "#",
                        class: "kh-text-gradient",
                        style: "font-size: var(--text-lg); font-weight: var(--font-weight-semibold);",
                        "KoloHub"
                    }
                    button { class: "kh-btn-outline",
                        style: "height: 2.5rem; padding: 0 0.875rem;",
                        "Toggle (Phase 4)"
                    }
                }
            }

            main {
                style: "max-width: 64rem; margin: 0 auto; padding: 7.5rem 1rem 4rem; text-align: center;",
                h1 { class: "kh-text-gradient",
                    style: "font-size: var(--text-5xl); font-weight: var(--font-weight-bold); line-height: var(--leading-tight); margin: 0 0 1rem 0;",
                    "Purple marketplace.",
                    br {}
                    "Local commerce."
                }
                p {
                    style: "margin: 0 auto; max-width: 42rem; color: var(--color-text-secondary); font-size: var(--text-lg);",
                    "Phase 3 — shell layout + glass nav. Brand purple from Rust: {brand}"
                }
                div { style: "display: flex; gap: 1rem; justify-content: center; margin-top: 2rem; flex-wrap: wrap;",
                    a { href: "#", class: "kh-btn-brand", "Get Started" }
                    a { href: "#", class: "kh-btn-accent", "Shop Now" }
                }
            }
        }
    }
}
