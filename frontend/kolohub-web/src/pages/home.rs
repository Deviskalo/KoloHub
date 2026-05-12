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
            // Hero Section
            div { class: "kh-storefront-hero",
                div { class: "kh-storefront-content",
                    h1 { class: "kh-storefront-title", "Shop the best of Liberia in one place." }
                    p { class: "kh-storefront-sub", "Local vendors. Instant payments. Fast delivery." }
                    form {
                        class: "kh-search-wrapper",
                        onsubmit: move |evt| {
                            evt.prevent_default();
                            nav.push(Route::Browse {
                                query: search_query(),
                            });
                        },
                        input {
                            class: "kh-search-input",
                            placeholder: "Search for electronics, food, fashion...",
                            value: "{search_query}",
                            oninput: move |evt| search_query.set(evt.value()),
                        }
                        button { r#type: "submit", class: "kh-search-btn", "Search" }
                    }
                }
            }

            // Categories Slider
            h2 { style: "font-size: var(--text-xl); font-weight: var(--font-weight-bold); margin: 0 0 1rem; color: var(--color-text-primary);",
                "Top Categories"
            }
            div { class: "kh-cat-scroll",
                for (name , icon) in categories {
                    a { href: "/browse", class: "kh-cat-card",
                        span { class: "kh-cat-icon", "{icon}" }
                        span { class: "kh-cat-label", "{name}" }
                    }
                }
            }

            // Trending Products
            div { style: "display: flex; justify-content: space-between; align-items: center; margin: 3rem 0 1.5rem;",
                h2 { style: "font-size: var(--text-xl); font-weight: var(--font-weight-bold); margin: 0; color: var(--color-text-primary);",
                    "Trending Now"
                }
                a {
                    href: "/browse",
                    style: "font-size: var(--text-sm); font-weight: var(--font-weight-semibold); color: var(--color-brand); text-decoration: none;",
                    "View all →"
                }
            }
            // Show top 10 items instead of 4
            div { class: "kh-product-grid",
                for p in MOCK_PRODUCTS.iter().take(10) {
                    ProductCard { product: p.clone() }
                }
            }

            // Become a Vendor CTA
            div { class: "kh-vendor-cta",
                h2 { "Grow your business online." }
                p {
                    "Join hundreds of local Liberian merchants selling to nationwide customers through KoloHub."
                }
                a {
                    href: "/dashboard",
                    class: "kh-btn-brand kh-btn-lg",
                    style: "text-decoration: none; display: inline-flex; align-items: center;",
                    "Sell on KoloHub"
                }
            }
        }
    }
}
