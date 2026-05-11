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

/// App-level responsive helpers for Phase 5.2 layout fidelity.
const APP_CSS: &str = r#"
.kh-preview-grid {
  display: grid;
  grid-template-columns: 1fr;
  gap: 1rem;
  align-items: stretch;
}
.kh-dashboard-grid {
  display: grid;
  grid-template-columns: 1fr;
  gap: 1rem;
}
.kh-topbar-nav {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto;
  align-items: center;
  gap: 0.55rem;
}
.kh-main-shell {
  max-width: 72rem;
  margin: 0 auto;
  padding: 10.25rem 1rem 2rem;
}
.kh-brand-group {
  display: flex;
  align-items: center;
  gap: 0.6rem;
  min-width: 0;
}
.kh-desktop-sep {
  display: none;
}
.kh-nav-links {
  display: none;
  align-items: center;
  gap: 0.2rem;
  overflow-x: auto;
  max-width: 100%;
}
.kh-top-controls {
  display: flex;
  align-items: center;
  gap: 0.4rem;
}
.kh-mobile-menu-btn {
  height: 2.5rem;
  width: 2.5rem;
  border-radius: var(--radius-md);
  border: 1px solid var(--color-border-subtle);
  background: var(--color-surface-elevated);
  color: var(--color-text-primary);
  font-size: 1.05rem;
  line-height: 1;
}
.kh-mobile-backdrop {
  position: fixed;
  inset: 0;
  background: rgb(2 6 23 / 0.36);
  z-index: 70;
}
.kh-mobile-drawer {
  position: fixed;
  top: 0;
  right: 0;
  width: min(84vw, 340px);
  height: 100dvh;
  z-index: 80;
  background: var(--color-surface-elevated);
  border-left: 1px solid var(--color-border-subtle);
  box-shadow: -14px 0 35px rgb(0 0 0 / 0.24);
  padding: 0.95rem 0.75rem;
  display: grid;
  grid-template-rows: auto 1fr auto;
  gap: 0.7rem;
  animation: kh-drawer-in 180ms ease-out;
}
.kh-mobile-drawer-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.6rem;
}
.kh-mobile-drawer-title {
  margin: 0;
  color: var(--color-text-primary);
  font-size: var(--text-base);
  font-weight: var(--font-weight-semibold);
}
@keyframes kh-drawer-in {
  from {
    transform: translateX(40px);
    opacity: 0.6;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
}
.kh-actions-row {
  display: none;
  gap: 0.5rem;
  margin-left: auto;
}
.kh-actions-row a { white-space: nowrap; }
.kh-mobile-menu-panel {
  grid-column: 1 / -1;
  border: 1px solid var(--color-border-subtle);
  border-radius: var(--radius-lg);
  background: var(--color-surface-elevated);
  padding: 0.65rem;
  display: grid;
  gap: 0.5rem;
}
.kh-mobile-menu-links {
  display: grid;
  grid-template-columns: 1fr;
  grid-auto-rows: min-content;
  gap: 0.5rem;
  align-content: start;
  align-items: start;
  overflow-y: auto;
  min-height: 0;
}
.kh-mobile-menu-link {
  display: block;
  border: 1px solid var(--color-border-subtle);
  border-radius: var(--radius-md);
  padding: 0.62rem 0.78rem;
  color: var(--color-text-primary);
  text-decoration: none;
  font-size: var(--text-sm);
  line-height: 1.2;
  font-weight: var(--font-weight-medium);
  background: var(--color-surface);
  box-shadow: 0 1px 0 rgb(0 0 0 / 0.03);
  transition: transform 120ms ease, border-color 120ms ease, background-color 120ms ease;
}
.kh-mobile-menu-link:hover {
  border-color: var(--color-border-strong);
  background: var(--color-brand-muted);
  transform: translateY(-1px);
}
.kh-mobile-menu-link:first-child {
  border-color: var(--color-border-strong);
  background: var(--color-brand-muted);
  color: var(--color-brand-foreground);
}
.kh-mobile-menu-actions {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 0.45rem;
  margin-bottom: 0.6rem;
}
.kh-mobile-menu-actions .kh-btn-brand,
.kh-mobile-menu-actions .kh-btn-outline {
  height: 2.35rem;
  padding: 0 0.65rem;
  font-size: var(--text-xs);
  border-radius: var(--radius-md);
  justify-content: center;
}
.kh-preview-image {
  width: 100%;
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-card);
  border: 1px solid var(--color-border-subtle);
  background: var(--color-surface-elevated);
  object-fit: contain;
}
.kh-customer-flow {
  max-width: 72rem;
  margin: 0 auto;
  padding: 0 1rem 1.5rem;
}o
.kh-customer-flow .track {
  display: grid;
  grid-template-columns: 1fr;
  gap: 0.55rem;
}
.kh-customer-flow .step {
  border: 1px solid var(--color-border-subtle);
  border-radius: var(--radius-md);
  background: var(--color-surface-elevated);
  padding: 0.6rem 0.7rem;
}
.kh-customer-flow .step .label {
  margin: 0;
  color: var(--color-text-primary);
  font-size: var(--text-sm);
  font-weight: var(--font-weight-semibold);
}
.kh-customer-flow .step .meta {
  margin: 0.18rem 0 0;
  color: var(--color-text-secondary);
  font-size: var(--text-xs);
}
.kh-stat-grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 0.5rem;
}
.kh-stat-chip {
  border: 1px solid var(--color-border-subtle);
  border-radius: var(--radius-md);
  background: var(--color-surface);
  padding: 0.45rem 0.55rem;
  min-height: 3.05rem;
}
.kh-stat-chip .label {
  margin: 0;
  font-size: var(--text-xs);
  color: var(--color-text-muted);
}
.kh-stat-chip .value {
  margin: 0.1rem 0 0;
  font-size: var(--text-base);
  font-weight: var(--font-weight-semibold);
  color: var(--color-text-primary);
}
@media (max-width: 480px) {
  .kh-mobile-menu-actions { grid-template-columns: 1fr; }
  .kh-brand-group a {
    font-size: var(--text-base) !important;
  }
}
@media (min-width: 980px) {
  .kh-main-shell {
    padding-top: 7rem;
  }
  .kh-topbar-nav {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
  }
  .kh-brand-group {
    min-width: unset;
  }
  .kh-desktop-sep {
    display: block;
    width: 1px;
    height: 1.75rem;
    background: var(--color-border-subtle);
    flex: 0 0 auto;
  }
  .kh-nav-links {
    display: flex;
  }
  .kh-top-controls .kh-mobile-menu-btn {
    display: none;
  }
  .kh-mobile-backdrop,
  .kh-mobile-drawer {
    display: none;
  }
  .kh-actions-row {
    display: flex;
  }
  .kh-preview-grid {
    grid-template-columns: minmax(280px, 0.9fr) minmax(0, 1.7fr);
    gap: 1.25rem;
  }
  .kh-dashboard-grid {
    grid-template-columns: 1fr 1fr;
  }
  .kh-customer-flow .track {
    grid-template-columns: repeat(5, minmax(0, 1fr));
    gap: 0.7rem;
  }
}
"#;

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
fn TopBar(is_dark: bool, on_toggle: EventHandler<MouseEvent>) -> Element {
    let mut menu_open = use_signal(|| false);
    let toggle_label = if is_dark {
        "Switch to light mode"
    } else {
        "Switch to dark mode"
    };
    let theme_icon = if is_dark { "☀" } else { "☾" };
    let menu_label = if menu_open() {
        "Close menu"
    } else {
        "Open menu"
    };

    rsx! {
        div { style: "position: fixed; top: 1rem; left: 50%; transform: translateX(-50%); z-index: 50; width: 100%; max-width: 72rem; padding: 0 1rem;",
            nav {
                class: "kh-surface-glass kh-topbar-nav",
                style: "border-radius: var(--radius-full); padding: 0.5rem 0.75rem;",
                div { class: "kh-brand-group",
                    img {
                        src: "/images/logo-main.jpeg",
                        alt: "KoloHub logo",
                        style: "width: 2rem; height: 2rem; border-radius: var(--radius-full); object-fit: cover;",
                    }
                    a {
                        href: "#",
                        class: "kh-text-gradient",
                        style: "font-size: var(--text-lg); font-weight: var(--font-weight-semibold);",
                        "KoloHub"
                    }
                }
                div { class: "kh-top-controls",
                    button {
                        class: "kh-btn-outline",
                        style: "height: 2.5rem; width: 2.5rem; padding: 0; font-size: 1.1rem; line-height: 1;",
                        onclick: move |evt| on_toggle.call(evt),
                        aria_label: "{toggle_label}",
                        title: "{toggle_label}",
                        "{theme_icon}"
                    }
                    button {
                        class: "kh-mobile-menu-btn",
                        onclick: move |_| menu_open.set(!menu_open()),
                        aria_label: "{menu_label}",
                        title: "{menu_label}",
                        if menu_open() {
                            "✕"
                        } else {
                            "☰"
                        }
                    }
                }
                span { class: "kh-desktop-sep", aria_hidden: "true" }

                div { class: "kh-nav-links",
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
                span { class: "kh-desktop-sep", aria_hidden: "true" }

                div { class: "kh-actions-row",
                    a {
                        href: "#",
                        class: "kh-btn-brand",
                        style: "height: 2.5rem; padding: 0 0.875rem;",
                        "Customer App"
                    }
                    a {
                        href: "#",
                        class: "kh-btn-outline",
                        style: "height: 2.5rem; padding: 0 0.875rem;",
                        "Vendor & Admin"
                    }
                }
            }
        }
        if menu_open() {
            div {
                class: "kh-mobile-backdrop",
                onclick: move |_| menu_open.set(false),
            }
            aside { class: "kh-mobile-drawer",
                div { class: "kh-mobile-drawer-head",
                    p { class: "kh-mobile-drawer-title", "Menu" }
                    button {
                        class: "kh-btn-outline",
                        style: "height: 2.2rem; width: 2.2rem; padding: 0;",
                        onclick: move |_| menu_open.set(false),
                        aria_label: "Close menu",
                        "✕"
                    }
                }
                div { class: "kh-mobile-menu-links",
                    a {
                        href: "#",
                        class: "kh-mobile-menu-link",
                        onclick: move |_| menu_open.set(false),
                        "Browse Products"
                    }
                    a {
                        href: "#",
                        class: "kh-mobile-menu-link",
                        onclick: move |_| menu_open.set(false),
                        "Product Details"
                    }
                    a {
                        href: "#",
                        class: "kh-mobile-menu-link",
                        onclick: move |_| menu_open.set(false),
                        "Cart"
                    }
                    a {
                        href: "#",
                        class: "kh-mobile-menu-link",
                        onclick: move |_| menu_open.set(false),
                        "Checkout & Pay"
                    }
                    a {
                        href: "#",
                        class: "kh-mobile-menu-link",
                        onclick: move |_| menu_open.set(false),
                        "Track Delivery"
                    }
                }
                div { class: "kh-mobile-menu-actions",
                    a {
                        href: "#",
                        class: "kh-btn-brand",
                        onclick: move |_| menu_open.set(false),
                        "Customer App"
                    }
                    a {
                        href: "#",
                        class: "kh-btn-outline",
                        onclick: move |_| menu_open.set(false),
                        "Vendor & Admin"
                    }
                }
            }
        }
    }
}

#[component]
fn CustomerFlowStrip() -> Element {
    rsx! {
        section { class: "kh-customer-flow",
            div { style: "background: var(--color-surface-elevated); border: 1px solid var(--color-border-subtle); border-radius: var(--radius-xl); box-shadow: var(--shadow-card); padding: 0.9rem;",
                h2 { style: "margin: 0 0 0.75rem 0; color: var(--color-text-primary);",
                    "Customer Journey Flow"
                }
                div { class: "track",
                    div { class: "step",
                        p { class: "label", "1. Browse Products" }
                        p { class: "meta", "Categories, search, recommendations" }
                    }
                    div { class: "step",
                        p { class: "label", "2. Add to Cart" }
                        p { class: "meta", "Quantity updates and item review" }
                    }
                    div { class: "step",
                        p { class: "label", "3. Checkout & Pay" }
                        p { class: "meta", "Mobile Money, COD, or card" }
                    }
                    div { class: "step",
                        p { class: "label", "4. Track Delivery" }
                        p { class: "meta", "Status stages + tracking ID" }
                    }
                    div { class: "step",
                        p { class: "label", "5. Rate & Review" }
                        p { class: "meta", "Post-delivery trust loop" }
                    }
                }
            }
        }
    }
}

#[component]
fn MarketplacePreview(brand: &'static str) -> Element {
    rsx! {
        main { class: "kh-main-shell",
            div { class: "kh-preview-grid",
                section {
                    class: "kh-surface-glass",
                    style: "border-radius: var(--radius-xl); padding: 1.25rem; display: flex; flex-direction: column;",
                    h1 { style: "font-size: var(--text-4xl); font-weight: var(--font-weight-bold); margin: 0 0 0.5rem 0; color: var(--color-text-primary);",
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
                    div { style: "margin-top: auto; padding-top: 1rem; display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 0.5rem;",
                        div { style: "border: 1px solid var(--color-border-subtle); border-radius: var(--radius-md); padding: 0.5rem;",
                            p { style: "margin: 0; color: var(--color-text-muted); font-size: var(--text-xs);",
                                "Currency"
                            }
                            p { style: "margin: 0.2rem 0 0; color: var(--color-text-primary); font-weight: var(--font-weight-semibold);",
                                "UGX"
                            }
                        }
                        div { style: "border: 1px solid var(--color-border-subtle); border-radius: var(--radius-md); padding: 0.5rem;",
                            p { style: "margin: 0; color: var(--color-text-muted); font-size: var(--text-xs);",
                                "Flow Stages"
                            }
                            p { style: "margin: 0.2rem 0 0; color: var(--color-text-primary); font-weight: var(--font-weight-semibold);",
                                "Browse → Track"
                            }
                        }
                    }
                }

                img {
                    src: "/images/design-ui.png",
                    alt: "KoloHub Design UI preview",
                    class: "kh-preview-image",
                }
            }
        }
    }
}

#[component]
fn DashboardHighlights() -> Element {
    rsx! {
        section { style: "max-width: 72rem; margin: 0 auto; padding: 0 1rem 3rem;",
            div { class: "kh-dashboard-grid",
                article { style: "background: var(--color-surface-elevated); border: 1px solid var(--color-border-subtle); border-radius: var(--radius-xl); box-shadow: var(--shadow-card); padding: 1rem;",
                    div { style: "display: flex; align-items: center; justify-content: space-between; gap: 0.75rem; margin-bottom: 0.5rem;",
                        h2 { style: "margin: 0; color: var(--color-text-primary);",
                            "Vendor Dashboard"
                        }
                        span { style: "font-size: var(--text-xs); color: var(--color-accent); font-weight: var(--font-weight-semibold);",
                            "+12.5%"
                        }
                    }
                    p { style: "margin: 0 0 0.9rem 0; color: var(--color-text-secondary);",
                        "Sales, Orders, Customers, and Conversion snapshots."
                    }
                    div { class: "kh-stat-grid",
                        div { class: "kh-stat-chip",
                            p { class: "label", "Total Sales" }
                            p { class: "value", "UGX 2.45M" }
                        }
                        div { class: "kh-stat-chip",
                            p { class: "label", "Total Orders" }
                            p { class: "value", "58" }
                        }
                        div { class: "kh-stat-chip",
                            p { class: "label", "Conversion" }
                            p { class: "value", "8.6%" }
                        }
                    }
                }
                article { style: "background: var(--color-surface-elevated); border: 1px solid var(--color-border-subtle); border-radius: var(--radius-xl); box-shadow: var(--shadow-card); padding: 1rem;",
                    div { style: "display: flex; align-items: center; justify-content: space-between; gap: 0.75rem; margin-bottom: 0.5rem;",
                        h2 { style: "margin: 0; color: var(--color-text-primary);",
                            "Admin Dashboard"
                        }
                        span { style: "font-size: var(--text-xs); color: var(--color-accent); font-weight: var(--font-weight-semibold);",
                            "+18.0%"
                        }
                    }
                    p { style: "margin: 0 0 0.9rem 0; color: var(--color-text-secondary);",
                        "Platform users, vendors, orders, and revenue overview."
                    }
                    div { class: "kh-stat-grid",
                        div { class: "kh-stat-chip",
                            p { class: "label", "Total Users" }
                            p { class: "value", "12,540" }
                        }
                        div { class: "kh-stat-chip",
                            p { class: "label", "Total Vendors" }
                            p { class: "value", "3,245" }
                        }
                        div { class: "kh-stat-chip",
                            p { class: "label", "Revenue" }
                            p { class: "value", "UGX 450.2M" }
                        }
                    }
                }
            }
        }
    }
}
