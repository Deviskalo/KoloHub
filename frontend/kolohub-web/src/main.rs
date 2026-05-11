//! KoloHub web shell — phased work: see `docs/frontend-phases.md`.

mod components;
mod styles;

use components::{
    dashboard::DashboardHighlights, flow_strip::CustomerFlowStrip, preview::MarketplacePreview,
    top_bar::TopBar,
};
use dioxus::prelude::*;
use kolohub_theme::{palette::purple, ColorScheme};
use styles::{APP_CSS, TOKENS_CSS};

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
        document::Style { {TOKENS_CSS} }
        document::Style { {APP_CSS} }
        document::Title { "KoloHub — Modern, Fast, Mobile First" }
        document::Meta {
            name: "description",
            content: "KoloHub marketplace: customer app, vendor dashboard, and admin control center.",
        }
        document::Link { rel: "icon", r#type: "image/png", href: "/favicon.png" }
        div { class: "{root_class}",
            TopBar {
                is_dark: matches!(color_scheme(), ColorScheme::Dark),
                on_toggle: move |_| {
                    color_scheme.set(color_scheme().toggle());
                },
            }
            MarketplacePreview { brand }
            CustomerFlowStrip {}
            DashboardHighlights {}
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
    #[cfg(not(target_arch = "wasm32"))]
    let _ = scheme;

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
