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

    let mut elapsed_seconds = use_signal(|| 0_u32);

    // Background timer loop
    use_future(move || async move {
        loop {
            #[cfg(target_arch = "wasm32")]
            gloo_timers::future::TimeoutFuture::new(1000).await;

            #[cfg(not(target_arch = "wasm32"))]
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;

            elapsed_seconds += 1;
        }
    });

    let flash_deals = [
        (
            "Smart Watch Pro",
            "45.00",
            "https://images.unsplash.com/photo-1523275335684-37898b6baf30?w=400",
            "80%",
            15735_u32, // 04:22:15
        ),
        (
            "Leather Bag",
            "25.00",
            "https://images.unsplash.com/photo-1548036328-c9fa89d128fa?w=400",
            "45%",
            43500_u32, // 12:05:00
        ),
    ]
    .into_iter()
    .map(|(name, price, img, stock, duration)| {
        let total_sec = duration.saturating_sub(elapsed_seconds());
        let h = total_sec / 3600;
        let m = (total_sec % 3600) / 60;
        let s = total_sec % 60;
        (
            name,
            price,
            img,
            stock,
            format!("{:02}", h),
            format!("{:02}", m),
            format!("{:02}", s),
        )
    })
    .collect::<Vec<_>>();

    rsx! {
        main { class: "kh-main-shell",
            // Background Orbs for Depth
            div { class: "kh-glow-orb primary", style: "top: 10%; left: 10%;" }
            div {
                class: "kh-glow-orb accent",
                style: "top: 20%; right: 10%; animation-delay: -5s;",
            }

            // Hero Section — Modernized with UI Components & Advanced Search
            div { class: "kh-storefront-hero kh-hero-banner kh-animate-fade-in-up",
                // Floating Hero Elements
                span {
                    class: "kh-animate-float",
                    style: "position: absolute; top: 15%; left: 10%; font-size: 2.5rem; opacity: 0.8;",
                    "🛍️"
                }
                span {
                    class: "kh-animate-float",
                    style: "position: absolute; bottom: 20%; right: 15%; font-size: 2rem; animation-delay: 1.5s; opacity: 0.6;",
                    "📱"
                }
                span {
                    class: "kh-animate-float",
                    style: "position: absolute; top: 10%; right: 30%; font-size: 1.8rem; animation-delay: 0.8s; opacity: 0.4;",
                    "🚚"
                }

                div { class: "kh-storefront-content",
                    h1 {
                        class: "kh-storefront-title",
                        style: "animation-delay: 0.1s;",
                        "Everything Liberia, Delivered Instantly."
                    }
                    p {
                        class: "kh-storefront-sub",
                        style: "animation-delay: 0.2s;",
                        "Discover 1,000+ local products from trusted Monrovia vendors."
                    }

                    div {
                        class: "kh-search-container",
                        style: "animation-delay: 0.3s;",
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
                                    nav.push(Route::Browse {
                                        query: search_query(),
                                    });
                                },
                                "Search"
                            }
                        }

                        // Dynamic Trending Search Ticker
                        div { class: "kh-search-ticker",
                            span { class: "kh-ticker-label", "Trending" }
                            div { class: "kh-ticker-content",
                                div { class: "kh-ticker-item", "Latest iPhone 15" }
                                div { class: "kh-ticker-item", "Liberian Coffee Beans" }
                                div { class: "kh-ticker-item", "Nimba County Goods" }
                                div { class: "kh-ticker-item", "Lappa Fashion" }
                            }
                        }

                        // Advanced Search Dropdown (Appears on Focus)
                        div { class: "kh-search-dropdown",
                            p { style: "font-size: var(--text-xs); font-weight: var(--font-weight-bold); color: var(--color-text-muted); margin: 0 0 0.5rem 0.5rem; text-align: left;",
                                "POPULAR RIGTH NOW"
                            }
                            div { style: "display: grid; gap: 0.25rem;",
                                for item in ["Latest iPhone", "Liberian Coffee", "Lappa Suits", "Rice & Dry Goods"] {
                                    Link {
                                        to: Route::Browse {
                                            query: item.to_lowercase(),
                                        },
                                        class: "kh-search-suggestion",
                                        span { style: "opacity: 0.6;", "🔍" }
                                        span { "{item}" }
                                    }
                                }
                            }
                        }
                    }

                    // County Hubs (Regional Discovery)
                    div {
                        class: "kh-county-strip",
                        style: "animation-delay: 0.4s;",
                        span { style: "font-size: var(--text-xs); opacity: 0.7; color: #fff; align-self: center;",
                            "Shop by Region: "
                        }
                        for county in ["Montserrado", "Nimba", "Grand Bassa", "Lofa"] {
                            div { class: "kh-county-chip", "{county}" }
                        }
                    }
                }
            }

            // Categories Slider
            div { style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 1.5rem;",
                h2 { style: "font-size: var(--text-xl); font-weight: var(--font-weight-bold); margin: 0; color: var(--color-text-primary);",
                    "Explore Categories"
                }
            }
            div {
                class: "kh-cat-scroll kh-animate-fade-in-up",
                style: "animation-delay: 0.5s;",
                for (name , icon) in categories {
                    a {
                        href: Route::CategoryPage {
                            name: name.to_string(),
                        }
                            .to_string(),
                        class: "kh-cat-card kh-surface-glass",
                        style: "transition: transform 0.3s ease; box-shadow: var(--shadow-sm);",
                        span { class: "kh-cat-icon", "{icon}" }
                        span { class: "kh-cat-label", "{name}" }
                    }
                }
            }

            // Section: Flash Deals (Newly Added)
            div {
                class: "kh-animate-fade-in-up",
                style: "animation-delay: 0.6s;",
                div { style: "display: flex; justify-content: space-between; align-items: center; margin: 3rem 0 1.5rem;",
                    div {
                        h2 { style: "font-size: var(--text-xl); font-weight: var(--font-weight-bold); margin: 0; color: var(--color-brand);",
                            "🔥 Flash Deals"
                        }
                        p { style: "font-size: var(--text-sm); color: var(--color-text-secondary); margin: 0.25rem 0 0;",
                            "Limited time offers. Act fast!"
                        }
                    }
                }
                div { class: "kh-flash-grid",
                    for (name , price , img , stock , h , m , s) in flash_deals {
                        div { class: "kh-flash-card kh-surface-glass",
                            div { class: "kh-flash-badge", "SAVE 30%" }
                            img { class: "kh-flash-img", src: "{img}" }
                            div { class: "kh-flash-info",
                                h4 { style: "margin: 0; font-size: var(--text-base);",
                                    "{name}"
                                }
                                p { style: "margin: 0.25rem 0; font-weight: bold; color: var(--color-brand);",
                                    "${price}"
                                }
                                div { class: "kh-progress-bar",
                                    div {
                                        class: "kh-progress-fill",
                                        style: "width: {stock};",
                                    }
                                }
                                p { style: "font-size: 0.65rem; color: var(--color-text-muted); margin: 0.25rem 0 0;",
                                    "Sold: {stock}"
                                }
                                div { class: if h == "00" { "kh-countdown kh-animate-pulse" } else { "kh-countdown" },
                                    span { class: "kh-countdown-unit", "{h}" }
                                    span { style: "font-size: 0.7rem; color: var(--color-brand);",
                                        ":"
                                    }
                                    span { class: "kh-countdown-unit", "{m}" }
                                    span { style: "font-size: 0.7rem; color: var(--color-brand);",
                                        ":"
                                    }
                                    span { class: "kh-countdown-unit", "{s}" }
                                }
                            }
                        }
                    }
                }
            }

            // Section 1: Trending Now
            div {
                class: "kh-animate-fade-in-up",
                style: "animation-delay: 0.7s;",
                div { style: "display: flex; justify-content: space-between; align-items: center; margin: 4rem 0 1.5rem;",
                    div {
                        h2 { style: "font-size: var(--text-xl); font-weight: var(--font-weight-bold); margin: 0; color: var(--color-text-primary);",
                            "Trending Now"
                        }
                        p { style: "font-size: var(--text-sm); color: var(--color-text-secondary); margin: 0.25rem 0 0;",
                            "Most popular items this week in Monrovia."
                        }
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
            }

            // Section 2: Market Favorites (Newly Added)
            div {
                class: "kh-animate-fade-in-up",
                style: "animation-delay: 0.9s;",
                div { style: "display: flex; justify-content: space-between; align-items: center; margin: 4rem 0 1.5rem;",
                    div {
                        h2 { style: "font-size: var(--text-xl); font-weight: var(--font-weight-bold); margin: 0; color: var(--color-text-primary);",
                            "Market Favorites"
                        }
                        p { style: "font-size: var(--text-sm); color: var(--color-text-secondary); margin: 0.25rem 0 0;",
                            "Top-rated local selections."
                        }
                    }
                }
                div { class: "kh-product-grid",
                    for p in MOCK_PRODUCTS.iter().skip(5).take(5) {
                        ProductCard { product: p.clone() }
                    }
                }
            }

            // Vendor Spotlight Section (Newly Added)
            div {
                class: "kh-animate-fade-in-up",
                style: "animation-delay: 1.0s;",
                div { class: "kh-vendor-spotlight",
                    div { class: "kh-vendor-profile-card",
                        div { class: "kh-vendor-avatar-row",
                            img {
                                class: "kh-vendor-avatar",
                                src: "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=200",
                            }
                            div {
                                h3 { style: "margin: 0; font-size: var(--text-xl);",
                                    "Mamie's Dry Goods"
                                }
                                p { style: "margin: 0.15rem 0 0; color: var(--color-text-secondary); font-size: var(--text-sm);",
                                    "Trusted Vendor since 2024"
                                }
                            }
                        }
                        p { style: "color: var(--color-text-secondary); line-height: 1.6;",
                            "Specializing in authentic Liberian dry goods, coffee, and seasonings. We source directly from local farmers in Nimba and Lofa counties."
                        }
                        div { class: "kh-vendor-stats",
                            div { class: "kh-vendor-stat-item",
                                span { class: "kh-vendor-stat-val", "4.9/5" }
                                span { class: "kh-vendor-stat-label", "Rating" }
                            }
                            div { class: "kh-vendor-stat-item",
                                span { class: "kh-vendor-stat-val", "1.2k+" }
                                span { class: "kh-vendor-stat-label", "Orders" }
                            }
                        }
                        crate::components::ui::Button { variant: "outline", "View Shop Profile" }
                    }
                    div {
                        h2 { style: "font-size: 2rem; font-weight: var(--font-weight-bold); line-height: 1.2;",
                            "Empowering local vendors to reach a national audience."
                        }
                        p { style: "margin: 1.5rem 0 2rem; color: var(--color-text-secondary); font-size: var(--text-lg); line-height: 1.5;",
                            "KoloHub isn't just a shop; it's a platform for Liberian entrepreneurs. We handle the logistics so you can focus on your craft."
                        }
                        crate::components::ui::Button { "Join as a Vendor" }
                    }
                }
            }

            // Trust Strip
            div {
                class: "kh-animate-fade-in-up",
                style: "animation-delay: 1.1s; margin: 5rem 0; display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 2rem; padding: 3rem; background: var(--color-surface-elevated); border-radius: var(--radius-xl); border: 1px solid var(--color-border-subtle);",
                div { class: "kh-text-center",
                    span { style: "font-size: 2rem;", "🚚" }
                    h4 { style: "margin: 0.5rem 0 0.25rem;", "Fast Delivery" }
                    p { style: "font-size: var(--text-sm); color: var(--color-text-secondary);",
                        "Monrovia-wide within 24h"
                    }
                }
                div { class: "kh-text-center",
                    span { style: "font-size: 2rem;", "🛡️" }
                    h4 { style: "margin: 0.5rem 0 0.25rem;", "Buyer Protection" }
                    p { style: "font-size: var(--text-sm); color: var(--color-text-secondary);",
                        "Guaranteed refunds for issues"
                    }
                }
                div { class: "kh-text-center",
                    span { style: "font-size: 2rem;", "💳" }
                    h4 { style: "margin: 0.5rem 0 0.25rem;", "Easy Payments" }
                    p { style: "font-size: var(--text-sm); color: var(--color-text-secondary);",
                        "Mobile Money or Cash"
                    }
                }
            }

            // Section: Community Voice (Newly Added)
            div {
                class: "kh-animate-fade-in-up",
                style: "animation-delay: 1.3s;",
                div { style: "text-align: center; margin: 6rem 0 3rem;",
                    h2 { style: "font-size: 2rem; font-weight: var(--font-weight-bold);",
                        "What our community says"
                    }
                    p { style: "color: var(--color-text-secondary);",
                        "Join thousands of happy shoppers across Liberia."
                    }
                }
                div { class: "kh-testimonial-grid",
                    for (name , role , text , img) in [
                        (
                            "Princess K.",
                            "Shopper from Gbarnga",
                            "I found authentic Lappa suits for my whole family. The delivery was faster than I expected!",
                            "https://images.unsplash.com/photo-1531123897727-8f129e1688ce?w=100",
                        ),
                        (
                            "Dixon M.",
                            "Business Owner",
                            "Selling on KoloHub changed my business. I can now reach customers in Monrovia from my shop in Nimba.",
                            "https://images.unsplash.com/photo-1506794778202-cad84cf45f1d?w=100",
                        ),
                    ]
                    {
                        div { class: "kh-testimonial-card",
                            p { style: "font-style: italic; color: var(--color-text-primary); line-height: 1.6;",
                                "\"{text}\""
                            }
                            div { class: "kh-testimonial-author",
                                img {
                                    class: "kh-testimonial-avatar",
                                    src: "{img}",
                                }
                                div {
                                    h4 { style: "margin: 0; font-size: var(--text-sm);",
                                        "{name}"
                                    }
                                    p { style: "margin: 0; font-size: var(--text-xs); color: var(--color-text-muted);",
                                        "{role}"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Section: Newsletter (Newly Added)
            div {
                class: "kh-newsletter-section kh-animate-fade-in-up",
                style: "animation-delay: 1.4s;",
                h2 { "Get the best of Liberia in your inbox" }
                p { style: "color: var(--color-text-secondary); margin-top: 0.5rem;",
                    "Subscribe for weekly deals, new vendor alerts, and more."
                }
                form {
                    class: "kh-newsletter-form",
                    onsubmit: move |evt| {
                        evt.prevent_default();
                    },
                    input {
                        r#type: "email",
                        class: "kh-input",
                        placeholder: "Enter your email address",
                        style: "flex: 1;",
                    }
                    crate::components::ui::Button { "Subscribe" }
                }
                p { style: "font-size: var(--text-xs); color: var(--color-text-muted); margin-top: 1.5rem;",
                    "We respect your privacy. Unsubscribe at any time."
                }
            }

            // Become a Vendor CTA — Modernized
            div {
                class: "kh-vendor-cta kh-surface-glass kh-animate-fade-in-up",
                style: "animation-delay: 1.5s; border: 1px solid var(--color-brand);",
                h2 { "Ready to take your business online?" }
                p {
                    "Join Liberia's largest digital marketplace. Manage your inventory, track sales, and reach thousands of customers effortlessly."
                }
                div { style: "display: flex; gap: 1rem; justify-content: center;",
                    crate::components::ui::Button {
                        onclick: move |_| {
                            nav.push(Route::VendorOverview {});
                        },
                        "Start Selling Today"
                    }
                    crate::components::ui::Button {
                        variant: "outline",
                        onclick: move |_| {
                            nav.push(Route::SellerGuidelines {});
                        },
                        "Read Guidelines"
                    }
                }
            }
        }
    }
}
