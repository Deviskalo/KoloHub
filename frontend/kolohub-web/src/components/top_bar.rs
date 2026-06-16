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
        div { style: "position: fixed; top: 1rem; left: 50%; transform: translateX(-50%); z-index: 50; width: 100%; max-width: 80rem; padding: 0 1rem;",
            nav {
                class: "kh-surface-glass kh-topbar-nav",
                style: "border-radius: var(--radius-full); padding: 0.5rem 0.75rem; display: flex; align-items: center; gap: 0.75rem; position: relative;",

                // Brand Section
                div { class: "kh-brand-group",
                    img {
                        src: "/logo.webp",
                        alt: "KoloHub logo",
                        style: "width: 2.25rem; height: 2.25rem; border-radius: var(--radius-full); object-fit: contain;",
                    }
                    a {
                        href: "/",
                        class: "kh-text-gradient",
                        style: "font-size: var(--text-lg); font-weight: var(--font-weight-bold);",
                        "KoloHub"
                    }
                }

                // Discovery Section with Dropdown
                div { class: "kh-category-wrapper",
                    button { class: "kh-btn-categories",
                        span { style: "font-size: 1.1rem;", "⊞" }
                        "Categories"
                    }
                    div { class: "kh-category-dropdown",
                        a { href: "/browse/produce", class: "kh-dropdown-item", "🥬 Fresh Produce" }
                        a { href: "/browse/electronics", class: "kh-dropdown-item", "📱 Electronics" }
                        a { href: "/browse/fashion", class: "kh-dropdown-item", "👕 Fashion & Apparel" }
                        a { href: "/browse/home", class: "kh-dropdown-item", "🏠 Home & Living" }
                        hr { style: "border: 0; border-top: 1px solid var(--color-border-subtle); margin: 0.5rem 0;" }
                        a { href: "/browse", class: "kh-dropdown-item", style: "font-weight: 600;", "View All Categories →" }
                    }
                }

                // Global Marketplace Search
                div { class: "kh-search-container",
                    span { class: "kh-search-icon", "🔍" }
                    input {
                        r#type: "text",
                        class: "kh-search-input",
                        placeholder: "Search for items, vendors, or services...",
                    }
                }

                // Utility Hub (Icons + Badges with better spacing)
                div { class: "kh-utility-hub",
                    a { href: "/account/track", class: "kh-nav-icon-btn", title: "Track Delivery",
                        span { style: "font-size: 1.25rem;", "🚚" }
                    }
                    a { href: "/cart", class: "kh-nav-icon-btn", title: "My Cart",
                        span { style: "font-size: 1.25rem;", "🛒" }
                        span { class: "kh-nav-badge", "2" }
                    }
                    button { class: "kh-nav-icon-btn", title: "Notifications",
                        span { style: "font-size: 1.25rem;", "🔔" }
                        span { class: "kh-nav-badge", "•" }
                    }
                }

                span { class: "kh-desktop-sep", aria_hidden: "true" }

                // Mode Switching & Theme
                div { class: "kh-actions-row",
                    a {
                        href: "/account",
                        class: "kh-btn-brand",
                        style: "height: 2.25rem; padding: 0 1rem; border-radius: var(--radius-full); font-size: var(--text-sm);",
                        "My Account"
                    }
                    a {
                        href: "/dashboard",
                        class: "kh-btn-outline",
                        style: "height: 2.25rem; padding: 0 1rem; border-radius: var(--radius-full); font-size: var(--text-sm);",
                        "Sell"
                    }
                    button {
                        class: "kh-nav-icon-btn",
                        style: "font-size: 1.1rem;",
                        onclick: move |evt| on_toggle.call(evt),
                        "{theme_icon}"
                    }
                }

                // Mobile Menu Toggle (Responsive Class)
                button {
                    class: "kh-mobile-menu-btn",
                    onclick: move |_| menu_open.set(!menu_open()),
                    if menu_open() { "✕" } else { "☰" }
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
                    p { class: "kh-mobile-drawer-title", "KoloHub Marketplace" }
                    button {
                        class: "kh-btn-outline",
                        style: "height: 2.2rem; width: 2.2rem; padding: 0;",
                        onclick: move |_| menu_open.set(false),
                        "✕"
                    }
                }

                // Mobile Search
                div { class: "kh-drawer-search",
                    div { class: "kh-search-container", style: "margin: 0; max-width: 100%;",
                        span { class: "kh-search-icon", "🔍" }
                        input {
                            r#type: "text",
                            class: "kh-search-input",
                            placeholder: "Search anything...",
                        }
                    }
                }

                div { class: "kh-mobile-menu-links",
                    a { href: "/browse", class: "kh-mobile-menu-link", onclick: move |_| menu_open.set(false),
                        span { style: "margin-right: 0.75rem;", "🛍️" }
                        "Browse All"
                    }
                    a { href: "/account/track", class: "kh-mobile-menu-link", onclick: move |_| menu_open.set(false),
                        span { style: "margin-right: 0.75rem;", "🚚" }
                        "Track My Orders"
                    }
                    a { href: "/cart", class: "kh-mobile-menu-link", onclick: move |_| menu_open.set(false),
                        span { style: "margin-right: 0.75rem;", "🛒" }
                        "Shopping Cart"
                    }
                }

                div { class: "kh-mobile-menu-actions",
                    a { href: "/account", class: "kh-btn-brand", onclick: move |_| menu_open.set(false),
                        "Buyer Portal"
                    }
                    a { href: "/dashboard", class: "kh-btn-outline", onclick: move |_| menu_open.set(false),
                        "Vendor Dashboard"
                    }
                }
            }
        }
    }
}
