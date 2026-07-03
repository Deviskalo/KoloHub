use crate::router::Route;
use dioxus::prelude::*;

#[component]
pub fn TopBar(is_dark: bool, on_toggle: EventHandler<MouseEvent>) -> Element {
    let mut menu_open = use_signal(|| false);
    let mut search_query = use_signal(|| "".to_string());
    let nav = use_navigator();

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
                        svg { class: "kh-icon-svg", view_box: "0 0 24 24",
                            rect { x: "3", y: "3", width: "7", height: "7" }
                            rect { x: "14", y: "3", width: "7", height: "7" }
                            rect { x: "14", y: "14", width: "7", height: "7" }
                            rect { x: "3", y: "14", width: "7", height: "7" }
                        }
                        "Categories"
                    }
                    div { class: "kh-category-dropdown",
                        a { href: "/browse/produce", class: "kh-dropdown-item", "🥬 Fresh Produce" }
                        a { href: "/browse/electronics", class: "kh-dropdown-item", "📱 Electronics" }
                        a { href: "/browse/fashion", class: "kh-dropdown-item", "👕 Fashion & Apparel" }
                        a { href: "/browse/home", class: "kh-dropdown-item", "🏠 Home & Living" }
                        hr { style: "border: 0; border-top: 1px solid var(--color-border-subtle); margin: 0.5rem 0;" }
                        a { href: "/categories", class: "kh-dropdown-item", style: "font-weight: 600;", "View All Categories →" }
                    }
                }
 
                // Global Marketplace Search - Wrapped in dynamic form
                form { 
                    class: "kh-search-container",
                    onsubmit: move |evt| {
                        evt.prevent_default();
                        let q = search_query();
                        if !q.trim().is_empty() {
                            nav.push(Route::Browse { query: q.trim().to_string() });
                        }
                    },
                    svg { class: "kh-icon-svg kh-search-icon", view_box: "0 0 24 24",
                        circle { cx: "11", cy: "11", r: "8" }
                        line { x1: "21", y1: "21", x2: "16.65", y2: "16.65" }
                    }
                    input {
                        r#type: "text",
                        class: "kh-search-input",
                        placeholder: "Search for items, vendors, or services...",
                        value: "{search_query}",
                        oninput: move |evt| search_query.set(evt.value()),
                    }
                }
 
                // Utility Hub (Icons + Badges with better spacing)
                div { class: "kh-utility-hub",
                    a { href: "/account/track", class: "kh-nav-icon-btn", title: "Track Delivery",
                        svg { class: "kh-icon-svg kh-icon-truck", view_box: "0 0 24 24",
                            rect { x: "1", y: "3", width: "15", height: "13" }
                            polygon { points: "16 8 20 8 23 11 23 16 16 16" }
                            circle { cx: "5.5", cy: "18.5", r: "2.5" }
                            circle { cx: "18.5", cy: "18.5", r: "2.5" }
                        }
                    }
                    a { href: "/cart", class: "kh-nav-icon-btn", title: "My Cart",
                        svg { class: "kh-icon-svg kh-icon-cart", view_box: "0 0 24 24",
                            circle { cx: "8", cy: "21", r: "1" }
                            circle { cx: "19", cy: "21", r: "1" }
                            path { d: "M2.05 2.05h2l2.66 12.42a2 2 0 0 0 2 1.58h9.78a2 2 0 0 0 1.95-1.57l1.65-7.43H5.12" }
                        }
                        span { class: "kh-nav-badge", "2" }
                    }
 
                    // Notification Hub
                    div { class: "kh-notif-wrapper",
                        button { class: "kh-nav-icon-btn", title: "Notifications",
                            svg { class: "kh-icon-svg kh-icon-bell", view_box: "0 0 24 24",
                                path { d: "M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9" }
                                path { d: "M13.73 21a2 2 0 0 1-3.46 0" }
                            }
                            span { class: "kh-nav-badge", "•" }
                        }
                        div { class: "kh-notif-dropdown",
                            div { class: "kh-notif-header",
                                span { class: "kh-notif-title", "Notifications" }
                                button { class: "kh-notif-clear", "Mark all as read" }
                            }
                            div { class: "kh-notif-list",
                                a { href: "/account/orders", class: "kh-notif-item unread",
                                    div { class: "kh-notif-icon order", "🚚" }
                                    div { class: "kh-notif-content",
                                        span { class: "kh-notif-msg", "Your order #KH-2940 is out for delivery." }
                                        span { class: "kh-notif-time", "2 mins ago" }
                                    }
                                }
                                a { href: "/browse", class: "kh-notif-item",
                                    div { class: "kh-notif-icon price", "🏷️" }
                                    div { class: "kh-notif-content",
                                        span { class: "kh-notif-msg", "Price drop! Organic Palm Oil is now $12.00." }
                                        span { class: "kh-notif-time", "1 hour ago" }
                                    }
                                }
                                a { href: "/account/security", class: "kh-notif-item",
                                    div { class: "kh-notif-icon security", "🛡️" }
                                    div { class: "kh-notif-content",
                                        span { class: "kh-notif-msg", "New login detected from Monrovia, LR." }
                                        span { class: "kh-notif-time", "Yesterday at 4:32 PM" }
                                    }
                                }
                            }
                            div { class: "kh-notif-footer",
                                a { href: "/account/notifications", class: "kh-notif-view-all", "View All Notifications →" }
                            }
                        }
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
                        aria_label: "{toggle_label}",
                        "{theme_icon}"
                    }
                }
 
                // Mobile Menu Toggle (Responsive Class)
                button {
                    class: "kh-mobile-menu-btn",
                    onclick: move |_| menu_open.set(!menu_open()),
                    aria_label: "{menu_label}",
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

                // Mobile Search - Wrapped in dynamic form
                form { 
                    class: "kh-drawer-search",
                    onsubmit: move |evt| {
                        evt.prevent_default();
                        let q = search_query();
                        if !q.trim().is_empty() {
                            menu_open.set(false);
                            nav.push(Route::Browse { query: q.trim().to_string() });
                        }
                    },
                    div { class: "kh-search-container", style: "margin: 0; max-width: 100%;",
                        svg { class: "kh-icon-svg kh-search-icon", view_box: "0 0 24 24",
                            circle { cx: "11", cy: "11", r: "8" }
                            line { x1: "21", y1: "21", x2: "16.65", y2: "16.65" }
                        }
                        input {
                            r#type: "text",
                            class: "kh-search-input",
                            placeholder: "Search anything...",
                            value: "{search_query}",
                            oninput: move |evt| search_query.set(evt.value()),
                        }
                    }
                }

                div { class: "kh-mobile-menu-links",
                    a { href: "/categories", class: "kh-mobile-menu-link", onclick: move |_| menu_open.set(false),
                        svg { class: "kh-icon-svg", style: "margin-right: 0.75rem; display: inline-block; vertical-align: middle;", view_box: "0 0 24 24",
                            path { d: "M6 2L3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6l-3-4z" }
                            line { x1: "3", y1: "6", x2: "21", y2: "6" }
                            path { d: "M16 10a4 4 0 0 1-8 0" }
                        }
                        "Browse Categories"
                    }
                    a { href: "/account/track", class: "kh-mobile-menu-link", onclick: move |_| menu_open.set(false),
                        svg { class: "kh-icon-svg", style: "margin-right: 0.75rem; display: inline-block; vertical-align: middle;", view_box: "0 0 24 24",
                            rect { x: "1", y: "3", width: "15", height: "13" }
                            polygon { points: "16 8 20 8 23 11 23 16 16 16" }
                            circle { cx: "5.5", cy: "18.5", r: "2.5" }
                            circle { cx: "18.5", cy: "18.5", r: "2.5" }
                        }
                        "Track My Orders"
                    }
                    a { href: "/cart", class: "kh-mobile-menu-link", onclick: move |_| menu_open.set(false),
                        svg { class: "kh-icon-svg", style: "margin-right: 0.75rem; display: inline-block; vertical-align: middle;", view_box: "0 0 24 24",
                            circle { cx: "8", cy: "21", r: "1" }
                            circle { cx: "19", cy: "21", r: "1" }
                            path { d: "M2.05 2.05h2l2.66 12.42a2 2 0 0 0 2 1.58h9.78a2 2 0 0 0 1.95-1.57l1.65-7.43H5.12" }
                        }
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
