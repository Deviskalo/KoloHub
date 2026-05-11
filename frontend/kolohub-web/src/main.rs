//! KoloHub web shell — phased work: see `docs/frontend-phases.md`.
//! Phase 2: global `tokens.css` in document head + root `kh-theme` wrapper.
//! Phase 3: shell layout with real brand assets + head metadata.
//! Phase 4: runtime theme toggle via `ColorScheme` + `dark` class.
//! Phase 4.1: persist theme in `localStorage` for web.

use dioxus::prelude::*;
use kolohub_theme::{palette::purple, ColorScheme};

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
    let mut color_scheme = use_signal(load_theme_from_storage);
    let brand = purple::P600;
    let root_class = match color_scheme() {
        ColorScheme::Light => "kh-theme",
        ColorScheme::Dark => "kh-theme dark",
    };

    use_effect(move || {
        save_theme_to_storage(color_scheme());
    });

    rsx! {
        document::Style {
            {TOKENS_CSS}
        }
        document::Title { "KoloHub — Modern, Fast, Mobile First" }
        document::Meta {
            name: "description",
            content: "KoloHub marketplace: customer app, vendor dashboard, and admin control center."
        }
        document::Link {
            rel: "icon",
            r#type: "image/png",
            href: "/favicon.png"
        }
        div { class: "{root_class}",
            TopNav {
                is_dark: matches!(color_scheme(), ColorScheme::Dark),
                on_toggle: move |_| {
                    color_scheme.set(color_scheme().toggle());
                }
            }
            HeroSection { brand: brand }
        }
    }
}

fn load_theme_from_storage() -> ColorScheme {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(value)) = storage.get_item("kolohub.theme") {
                    if value == "dark" {
                        return ColorScheme::Dark;
                    }
                }
            }
        }
    }
    ColorScheme::Light
}

fn save_theme_to_storage(scheme: ColorScheme) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let value = match scheme {
                    ColorScheme::Light => "light",
                    ColorScheme::Dark => "dark",
                };
                let _ = storage.set_item("kolohub.theme", value);
            }
        }
    }
}

#[component]
fn TopNav(
    is_dark: bool,
    on_toggle: EventHandler<MouseEvent>,
) -> Element {
    let toggle_label = if is_dark { "Light mode" } else { "Dark mode" };

    rsx! {
        div {
            style: "position: fixed; top: 1rem; left: 50%; transform: translateX(-50%); z-index: 50; width: 100%; max-width: 56rem; padding: 0 1rem;",
            nav { class: "kh-surface-glass",
                style: "display: flex; align-items: center; justify-content: space-between; border-radius: var(--radius-full); padding: 0.5rem 1rem;",
                div { style: "display: flex; align-items: center; gap: 0.75rem;",
                    img {
                        src: "/images/logo-main.jpeg",
                        alt: "KoloHub logo",
                        style: "width: 2rem; height: 2rem; border-radius: var(--radius-full); object-fit: cover;"
                    }
                    a {
                        href: "#",
                        class: "kh-text-gradient",
                        style: "font-size: var(--text-lg); font-weight: var(--font-weight-semibold);",
                        "KoloHub"
                    }
                }
                div { style: "display: flex; gap: 0.5rem;",
                    a { href: "#", class: "kh-btn-outline", style: "height: 2.5rem; padding: 0 0.875rem;", "Docs" }
                    button {
                        class: "kh-btn-outline",
                        style: "height: 2.5rem; padding: 0 0.875rem;",
                        onclick: move |evt| on_toggle.call(evt),
                        "{toggle_label}"
                    }
                }
            }
        }
    }
}

#[component]
fn HeroSection(brand: &'static str) -> Element {
    rsx! {
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
                "Phase 2 + 3 in one flow: global tokens, glass navigation, and visual assets are now integrated. Brand purple from Rust: {brand}"
            }
            div { style: "display: flex; gap: 1rem; justify-content: center; margin-top: 2rem; flex-wrap: wrap;",
                a { href: "#", class: "kh-btn-brand", "Get Started" }
                a { href: "#", class: "kh-btn-accent", "Shop Now" }
            }

            img {
                src: "/images/banner-desktop.jpeg",
                alt: "KoloHub platform preview",
                style: "display: block; width: 100%; max-width: 56rem; margin: 2rem auto 0 auto; border-radius: var(--radius-xl); box-shadow: var(--shadow-card); border: 1px solid var(--color-border-subtle);"
            }
        }
    }
}
