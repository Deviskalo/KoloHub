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
            TopBar {
                is_dark: matches!(color_scheme(), ColorScheme::Dark),
                on_toggle: move |_| {
                    color_scheme.set(color_scheme().toggle());
                }
            }
            MarketplacePreview { brand: brand }
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

fn save_theme_to_storage(_scheme: ColorScheme) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let value = match _scheme {
                    ColorScheme::Light => "light",
                    ColorScheme::Dark => "dark",
                };
                let _ = storage.set_item("kolohub.theme", value);
            }
        }
    }
}

#[component]
fn TopBar(
    is_dark: bool,
    on_toggle: EventHandler<MouseEvent>,
) -> Element {
    let toggle_label = if is_dark { "Switch to light mode" } else { "Switch to dark mode" };
    let theme_icon = if is_dark { "☀" } else { "☾" };

    rsx! {
        div {
            style: "position: fixed; top: 1rem; left: 50%; transform: translateX(-50%); z-index: 50; width: 100%; max-width: 72rem; padding: 0 1rem;",
            nav { class: "kh-surface-glass",
                style: "display: flex; flex-wrap: wrap; align-items: center; justify-content: space-between; gap: 0.75rem; border-radius: var(--radius-full); padding: 0.5rem 1rem;",
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
                span {
                    style: "width: 1px; height: 1.75rem; background: var(--color-border-subtle); flex: 0 0 auto;",
                    aria_hidden: "true"
                }

                div {
                    style: "display: flex; align-items: center; gap: 0.2rem; overflow-x: auto; max-width: 100%;",
                    a {
                        href: "#",
                        style: "height: 2.15rem; padding: 0 0.7rem; white-space: nowrap; display: inline-flex; align-items: center; border-radius: var(--radius-md); background: var(--color-brand-muted); color: var(--color-brand-foreground); font-size: var(--text-sm); font-weight: var(--font-weight-semibold); text-decoration: none;",
                        "Browse Products"
                    }
                    a {
                        href: "#",
                        style: "height: 2.15rem; padding: 0 0.7rem; white-space: nowrap; display: inline-flex; align-items: center; border-radius: var(--radius-md); color: var(--color-text-secondary); font-size: var(--text-sm); font-weight: var(--font-weight-medium); text-decoration: none;",
                        "Product Details"
                    }
                    a {
                        href: "#",
                        style: "height: 2.15rem; padding: 0 0.7rem; white-space: nowrap; display: inline-flex; align-items: center; border-radius: var(--radius-md); color: var(--color-text-secondary); font-size: var(--text-sm); font-weight: var(--font-weight-medium); text-decoration: none;",
                        "Cart"
                    }
                    a {
                        href: "#",
                        style: "height: 2.15rem; padding: 0 0.7rem; white-space: nowrap; display: inline-flex; align-items: center; border-radius: var(--radius-md); color: var(--color-text-secondary); font-size: var(--text-sm); font-weight: var(--font-weight-medium); text-decoration: none;",
                        "Checkout & Pay"
                    }
                    a {
                        href: "#",
                        style: "height: 2.15rem; padding: 0 0.7rem; white-space: nowrap; display: inline-flex; align-items: center; border-radius: var(--radius-md); color: var(--color-text-secondary); font-size: var(--text-sm); font-weight: var(--font-weight-medium); text-decoration: none;",
                        "Track Delivery"
                    }
                }
                span {
                    style: "width: 1px; height: 1.75rem; background: var(--color-border-subtle); flex: 0 0 auto;",
                    aria_hidden: "true"
                }

                div { style: "display: flex; gap: 0.5rem; margin-left: auto;",
                    a { href: "#", class: "kh-btn-brand", style: "height: 2.5rem; padding: 0 0.875rem;", "Customer App" }
                    a { href: "#", class: "kh-btn-outline", style: "height: 2.5rem; padding: 0 0.875rem;", "Vendor & Admin" }
                }
                span {
                    style: "width: 1px; height: 1.75rem; background: var(--color-border-subtle); flex: 0 0 auto;",
                    aria_hidden: "true"
                }
                div {
                    button {
                        class: "kh-btn-outline",
                        style: "height: 2.5rem; width: 2.5rem; padding: 0; font-size: 1.1rem; line-height: 1;",
                        onclick: move |evt| on_toggle.call(evt),
                        aria_label: "{toggle_label}",
                        title: "{toggle_label}",
                        "{theme_icon}"
                    }
                }
            }
        }
    }
}

#[component]
fn MarketplacePreview(brand: &'static str) -> Element {
    rsx! {
        main {
            style: "max-width: 72rem; margin: 0 auto; padding: 7rem 1rem 2rem;",
            div {
                style: "display: grid; grid-template-columns: 1fr; gap: 1rem; align-items: stretch;",
                section {
                    class: "kh-surface-glass",
                    style: "border-radius: var(--radius-xl); padding: 1.25rem;",
                    h1 {
                        style: "font-size: var(--text-4xl); font-weight: var(--font-weight-bold); margin: 0 0 0.5rem 0; color: var(--color-text-primary);",
                        "Frontend Preview"
                    }
                    p { style: "margin: 0 0 1rem 0; color: var(--color-text-secondary);",
                        "Modern, Fast + Mobile First"
                    }
                    p { style: "margin: 0; color: var(--color-text-secondary);",
                        "Built from Design-UI with design-system tokens. Brand purple: {brand}"
                    }
                    div { style: "display: flex; gap: 0.75rem; margin-top: 1rem; flex-wrap: wrap;",
                        a { href: "#", class: "kh-btn-accent", "Customer App" }
                        a { href: "#", class: "kh-btn-brand", "Vendor & Admin Panel" }
                    }
                }

                img {
                    src: "/images/design-ui.png",
                    alt: "KoloHub Design UI preview",
                    style: "width: 100%; border-radius: var(--radius-xl); box-shadow: var(--shadow-card); border: 1px solid var(--color-border-subtle); background: var(--color-surface-elevated);"
                }
            }
        }
    }
}

#[component]
fn DashboardHighlights() -> Element {
    rsx! {
        section {
            style: "max-width: 72rem; margin: 0 auto; padding: 0 1rem 3rem;",
            div {
                style: "display: grid; grid-template-columns: 1fr; gap: 1rem;",
                article {
                    style: "background: var(--color-surface-elevated); border: 1px solid var(--color-border-subtle); border-radius: var(--radius-xl); box-shadow: var(--shadow-card); padding: 1rem;",
                    h2 { style: "margin: 0 0 0.5rem 0; color: var(--color-text-primary);", "Vendor Dashboard" }
                    p { style: "margin: 0 0 0.75rem 0; color: var(--color-text-secondary);", "Sales, Orders, Customers, and Conversion snapshots." }
                    div { style: "display: flex; gap: 0.5rem; flex-wrap: wrap;",
                        span { class: "kh-btn-outline", style: "height: 2rem; padding: 0 0.75rem;", "UGX 2.45M Sales" }
                        span { class: "kh-btn-outline", style: "height: 2rem; padding: 0 0.75rem;", "58 Orders" }
                        span { class: "kh-btn-outline", style: "height: 2rem; padding: 0 0.75rem;", "8.6% Conversion" }
                    }
                }
                article {
                    style: "background: var(--color-surface-elevated); border: 1px solid var(--color-border-subtle); border-radius: var(--radius-xl); box-shadow: var(--shadow-card); padding: 1rem;",
                    h2 { style: "margin: 0 0 0.5rem 0; color: var(--color-text-primary);", "Admin Dashboard" }
                    p { style: "margin: 0 0 0.75rem 0; color: var(--color-text-secondary);", "Platform users, vendors, orders, and revenue overview." }
                    div { style: "display: flex; gap: 0.5rem; flex-wrap: wrap;",
                        span { class: "kh-btn-outline", style: "height: 2rem; padding: 0 0.75rem;", "12,540 Users" }
                        span { class: "kh-btn-outline", style: "height: 2rem; padding: 0 0.75rem;", "3,245 Vendors" }
                        span { class: "kh-btn-outline", style: "height: 2rem; padding: 0 0.75rem;", "UGX 450.2M Revenue" }
                    }
                }
            }
        }
    }
}
