//! Route definitions + AppLayout — the persistent shell wrapper for all pages.

use dioxus::prelude::*;
use kolohub_theme::ColorScheme;

use crate::components::top_bar::TopBar;
use crate::pages::{
    browse::Browse, cart::Cart, checkout::Checkout, home::Home, not_found::NotFound,
    product::Product, track::Track, vendor::Vendor,
};
use crate::theme::{load_theme_from_storage, save_theme_to_storage};

/// Top-level application routes.
#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        Home {},
        #[route("/browse")]
        Browse {},
        #[route("/product/:id")]
        Product { id: String },
        #[route("/cart")]
        Cart {},
        #[route("/checkout")]
        Checkout {},
        #[route("/track")]
        Track {},
        #[route("/vendor")]
        Vendor {},
    #[end_layout]
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

/// Persistent shell: theme signal + TopBar + page outlet.
#[component]
fn AppLayout() -> Element {
    let mut color_scheme = use_signal(load_theme_from_storage);
    let root_class = match color_scheme() {
        ColorScheme::Light => "kh-theme",
        ColorScheme::Dark => "kh-theme dark",
    };

    use_effect(move || {
        save_theme_to_storage(color_scheme());
    });

    rsx! {
        div { class: "{root_class}",
            TopBar {
                is_dark: matches!(color_scheme(), ColorScheme::Dark),
                on_toggle: move |_| {
                    color_scheme.set(color_scheme().toggle());
                },
            }
            Outlet::<Route> {}
        }
    }
}
