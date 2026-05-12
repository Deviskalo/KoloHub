use crate::pages::browse::{ProductCard, MOCK_PRODUCTS};
use crate::router::Route;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let mut search_query = use_signal(|| "".to_string());
    let nav = use_navigator();

    let categories = [
        ("Electronics", "📱"),
        ("Fashion", "👕"),
        ("Groceries", "🍅"),
        ("Home & Living", "🛋️"),
        ("Agribusiness", "🌾"),
        ("Health & Beauty", "💄"),
    ];

    rsx! {
        main { class: "kh-main-shell",
            // Hero Section — Modernized with UI Components
            div { class: "kh-storefront-hero kh-hero-banner",
                div { class: "kh-storefront-content",
                    h1 { class: "kh-storefront-title", "Everything Liberia, Delivered Instantly." }
                    p { class: "kh-storefront-sub", "Discover 1,000+ local products from trusted Monrovia vendors." }
                    
                    form {
                        class: "kh-search-wrapper kh-surface-glass",
                        onsubmit: move |evt| {
                            evt.prevent_default();
                            nav.push(Route::Browse {
                                query: search_query(),
                            });
                        },
                        input {
                            class: "kh-search-input",
                            style: "color: #fff; border: none; background: transparent;",
                            placeholder: "What are you looking for today?",
                            value: "{search_query}",
                            oninput: move |evt| search_query.set(evt.value()),
                        }
                        crate::components::ui::Button { 
                            onclick: move |_| {
                                nav.push(Route::Browse { query: search_query() });
                            },
                            "Search" 
                        }
                    }
                    
                    div { style: "margin-top: 1.5rem; display: flex; gap: 0.75rem; justify-content: center;",
                        span { style: "font-size: var(--text-xs); opacity: 0.8;", "Popular: " }
                        for tag in ["Electronics", "Phones", "Dry Goods"] {
                            a { 
                                href: "/browse?query={tag.to_lowercase()}",
                                style: "font-size: var(--text-xs); color: #fff; text-decoration: underline; opacity: 0.9;",
                                "{tag}"
                            }
                        }
                    }
                }
            }

            // Categories Slider
            div { style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 1.5rem;",
                h2 { style: "font-size: var(--text-xl); font-weight: var(--font-weight-bold); margin: 0; color: var(--color-text-primary);",
                    "Browse Categories"
                }
            }
            div { class: "kh-cat-scroll",
                for (name , icon) in categories {
                    a { 
                        href: "/browse?query={name.to_lowercase()}", 
                        class: "kh-cat-card kh-surface-glass",
                        span { class: "kh-cat-icon", "{icon}" }
                        span { class: "kh-cat-label", "{name}" }
                    }
                }
            }

            // Section 1: Trending Now
            div { style: "display: flex; justify-content: space-between; align-items: center; margin: 4rem 0 1.5rem;",
                div {
                    h2 { style: "font-size: var(--text-xl); font-weight: var(--font-weight-bold); margin: 0; color: var(--color-text-primary);",
                        "Trending Now"
                    }
                    p { style: "font-size: var(--text-sm); color: var(--color-text-secondary); margin: 0.25rem 0 0;", "Most popular items this week in Monrovia." }
                }
                a {
                    href: "/browse",
                    style: "font-size: var(--text-sm); font-weight: var(--font-weight-semibold); color: var(--color-brand); text-decoration: none;",
                    "View all →"
                }
            }
            div { class: "kh-product-grid",
                for p in MOCK_PRODUCTS.iter().take(5) {
                    ProductCard { product: p.clone() }
                }
            }

            // Section 2: Market Favorites (Newly Added)
            div { style: "display: flex; justify-content: space-between; align-items: center; margin: 4rem 0 1.5rem;",
                div {
                    h2 { style: "font-size: var(--text-xl); font-weight: var(--font-weight-bold); margin: 0; color: var(--color-text-primary);",
                        "Market Favorites"
                    }
                    p { style: "font-size: var(--text-sm); color: var(--color-text-secondary); margin: 0.25rem 0 0;", "Top-rated local selections." }
                }
            }
            div { class: "kh-product-grid",
                for p in MOCK_PRODUCTS.iter().skip(5).take(5) {
                    ProductCard { product: p.clone() }
                }
            }

            // Trust Strip
            div { 
                style: "margin: 5rem 0; display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 2rem; padding: 3rem; background: var(--color-surface-elevated); border-radius: var(--radius-xl); border: 1px solid var(--color-border-subtle);",
                div { class: "kh-text-center",
                    span { style: "font-size: 2rem;", "🚚" }
                    h4 { style: "margin: 0.5rem 0 0.25rem;", "Fast Delivery" }
                    p { style: "font-size: var(--text-sm); color: var(--color-text-secondary);", "Monrovia-wide within 24h" }
                }
                div { class: "kh-text-center",
                    span { style: "font-size: 2rem;", "🛡️" }
                    h4 { style: "margin: 0.5rem 0 0.25rem;", "Buyer Protection" }
                    p { style: "font-size: var(--text-sm); color: var(--color-text-secondary);", "Guaranteed refunds for issues" }
                }
                div { class: "kh-text-center",
                    span { style: "font-size: 2rem;", "💳" }
                    h4 { style: "margin: 0.5rem 0 0.25rem;", "Easy Payments" }
                    p { style: "font-size: var(--text-sm); color: var(--color-text-secondary);", "Mobile Money or Cash" }
                }
            }

            // Become a Vendor CTA — Modernized
            div { class: "kh-vendor-cta kh-surface-glass",
                style: "border: 1px solid var(--color-brand);",
                h2 { "Ready to take your business online?" }
                p {
                    "Join Liberia's largest digital marketplace. Manage your inventory, track sales, and reach thousands of customers effortlessly."
                }
                div { style: "display: flex; gap: 1rem; justify-content: center;",
                    crate::components::ui::Button { 
                        onclick: move |_| { nav.push(Route::VendorOverview {}); },
                        "Start Selling Today" 
                    }
                    crate::components::ui::Button { 
                        variant: "outline",
                        onclick: move |_| { nav.push(Route::SellerGuidelines {}); },
                        "Read Guidelines" 
                    }
                }
            }
        }
    }
}
