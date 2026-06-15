use dioxus::prelude::*;

#[component]
pub fn TopBar(is_dark: bool, on_toggle: EventHandler<MouseEvent>) -> Element {
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
                        src: "/logo.webp",
                        alt: "KoloHub logo",
                        style: "width: 2.5rem; height: 2.5rem; border-radius: var(--radius-full); object-fit: contain;",
                    }
                    a {
                        href: "/",
                        class: "kh-text-gradient",
                        style: "font-size: var(--text-lg); font-weight: var(--font-weight-semibold);",
                        "KoloHub"
                    }
                }

                span { class: "kh-desktop-sep", aria_hidden: "true" }

                div {
                    class: "kh-nav-links",
                    style: "justify-content: center; align-items: center; display: flex; ",
                    a {
                        href: "/browse",
                        style: "height: 2.15rem; padding: 0 0.7rem; white-space: nowrap; display: inline-flex; align-items: center; border-radius: var(--radius-md); background: var(--color-brand-muted); color: var(--color-brand-foreground); font-size: var(--text-sm); font-weight: var(--font-weight-semibold); text-decoration: none;",
                        "Browse Products"
                    }
                    a {
                        href: "/cart",
                        style: "height: 2.15rem; padding: 0 0.7rem; white-space: nowrap; display: inline-flex; align-items: center; border-radius: var(--radius-md); color: var(--color-text-secondary); font-size: var(--text-sm); font-weight: var(--font-weight-medium); text-decoration: none;",
                        "Cart"
                    }
                    a {
                        href: "/checkout",
                        style: "height: 2.15rem; padding: 0 0.7rem; white-space: nowrap; display: inline-flex; align-items: center; border-radius: var(--radius-md); color: var(--color-text-secondary); font-size: var(--text-sm); font-weight: var(--font-weight-medium); text-decoration: none;",
                        "Checkout & Pay"
                    }
                    a {
                        href: "/account/track",
                        style: "height: 2.15rem; padding: 0 0.7rem; white-space: nowrap; display: inline-flex; align-items: center; border-radius: var(--radius-md); color: var(--color-text-secondary); font-size: var(--text-sm); font-weight: var(--font-weight-medium); text-decoration: none;",
                        "Track Delivery"
                    }
                }
                span { class: "kh-desktop-sep", aria_hidden: "true" }

                div { class: "kh-actions-row",
                    a {
                        href: "/account",
                        class: "kh-btn-brand",
                        style: "height: 2.5rem; padding: 0 0.875rem;",
                        "My Account"
                    }
                    a {
                        href: "/dashboard",
                        class: "kh-btn-outline",
                        style: "height: 2.5rem; padding: 0 0.875rem;",
                        "Vendor & Admin"
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
                        href: "/browse",
                        class: "kh-mobile-menu-link",
                        onclick: move |_| menu_open.set(false),
                        "Browse Products"
                    }
                    a {
                        href: "/cart",
                        class: "kh-mobile-menu-link",
                        onclick: move |_| menu_open.set(false),
                        "Cart"
                    }
                    a {
                        href: "/checkout",
                        class: "kh-mobile-menu-link",
                        onclick: move |_| menu_open.set(false),
                        "Checkout & Pay"
                    }
                    a {
                        href: "/account/track",
                        class: "kh-mobile-menu-link",
                        onclick: move |_| menu_open.set(false),
                        "Track Delivery"
                    }
                }
                div { class: "kh-mobile-menu-actions",
                    a {
                        href: "/account",
                        class: "kh-btn-brand",
                        onclick: move |_| menu_open.set(false),
                        "My Account"
                    }
                    a {
                        href: "/dashboard",
                        class: "kh-btn-outline",
                        onclick: move |_| menu_open.set(false),
                        "Vendor & Admin"
                    }
                }
            }
        }
    }
}
