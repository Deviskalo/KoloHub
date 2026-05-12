use crate::pages::browse::MOCK_PRODUCTS;
use crate::router::Route;
use dioxus::prelude::*;

#[component]
pub fn Product(id: String) -> Element {
    let mut qty = use_signal(|| 1);
    let mut active_tab = use_signal(|| "description");
    
    // Find product in mocks
    let product = MOCK_PRODUCTS.iter().find(|p| p.id == id);

    if product.is_none() {
        return rsx! {
            main { class: "kh-main-shell",
                div { style: "text-align: center; padding: 10rem 2rem;",
                    h1 { "Product not found" }
                    Link { to: Route::Home {}, class: "kh-btn-brand", "Go Back Home" }
                }
            }
        };
    }

    let p = product.unwrap();

    rsx! {
        main { class: "kh-main-shell",
            div { class: "kh-animate-fade-in-up",
                // Breadcrumbs
                nav { style: "margin-bottom: 2rem; display: flex; gap: 0.5rem; font-size: var(--text-sm); color: var(--color-text-muted);",
                    Link { to: Route::Home {}, "Home" }
                    span { "/" }
                    Link { to: Route::Browse { query: "".to_string() }, "Browse" }
                    span { "/" }
                    span { style: "color: var(--color-text-primary);", "{p.title}" }
                }

                div { class: "kh-pdp-layout",
                    // Image Section
                    div {
                        img {
                            class: "kh-pdp-image-main",
                            src: "{p.image_url}",
                            alt: "{p.title}"
                        }
                        div { class: "kh-pdp-thumbnails",
                            for _ in 0..4 {
                                img {
                                    class: "kh-pdp-thumb",
                                    src: "{p.image_url}",
                                }
                            }
                        }
                    }

                    // Info Section
                    div { class: "kh-pdp-info-card",
                        span { class: "kh-flash-badge", style: "position: static;", "FREE DELIVERY" }
                        h1 { style: "font-size: 2.5rem; font-weight: var(--font-weight-bold); margin: 1rem 0 0.5rem;", "{p.title}" }
                        div { style: "display: flex; align-items: center; gap: 0.5rem; margin-bottom: 1.5rem;",
                            span { style: "color: #fbbf24;", "★★★★★" }
                            span { style: "font-size: var(--text-sm); color: var(--color-text-secondary);", "(24 Reviews)" }
                            span { style: "color: var(--color-border-subtle);", "|" }
                            span { style: "font-size: var(--text-sm); color: var(--color-brand); font-weight: bold;", "In Stock" }
                        }

                        div { class: "kh-pdp-price-row",
                            span { class: "kh-pdp-current-price", "{p.price}" }
                            span { class: "kh-pdp-old-price", "$65.00" }
                        }

                        p { style: "color: var(--color-text-secondary); line-height: 1.6; margin-bottom: 2rem;",
                            "Experience the best of Liberian quality with {p.title}. Sourced directly from {p.vendor}, ensuring you get authentic products while supporting local entrepreneurs."
                        }

                        div { class: "kh-qty-selector",
                            span { style: "font-weight: bold;", "Quantity" }
                            div { style: "display: flex; align-items: center; gap: 0.5rem;",
                                button {
                                    class: "kh-qty-btn",
                                    onclick: move |_| { if qty() > 1 { qty.set(qty() - 1) } },
                                    "-"
                                }
                                span { style: "width: 2rem; text-align: center; font-weight: bold;", "{qty}" }
                                button {
                                    class: "kh-qty-btn",
                                    onclick: move |_| { qty.set(qty() + 1) },
                                    "+"
                                }
                            }
                        }
                        {
                            let raw_price = p.price.replace('$', "").replace(',', "");
                            let unit_price = raw_price.parse::<f64>().unwrap_or(0.0);
                            let total = unit_price * (qty() as f64);
                            rsx! {
                                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-top: 2rem;",
                                    Link { 
                                        to: Route::Cart {},
                                        class: "kh-btn-brand",
                                        style: "text-align: center; display: flex; align-items: center; justify-content: center;",
                                        "Add to Cart — ${total:.2}" 
                                    }
                                    Link { 
                                        to: Route::Checkout {},
                                        class: "kh-btn-outline",
                                        style: "text-align: center; display: flex; align-items: center; justify-content: center;",
                                        "Buy Now — ${total:.2}" 
                                    }
                                }
                            }
                        }

                        div { class: "kh-pdp-features",
                            div { class: "kh-pdp-feature-item",
                                span { style: "font-size: 1.5rem;", "🚚" }
                                p { style: "margin: 0.5rem 0 0; font-size: 0.7rem;", "Fast Delivery" }
                            }
                            div { class: "kh-pdp-feature-item",
                                span { style: "font-size: 1.5rem;", "🛡️" }
                                p { style: "margin: 0.5rem 0 0; font-size: 0.7rem;", "Secure Pay" }
                            }
                            div { class: "kh-pdp-feature-item",
                                span { style: "font-size: 1.5rem;", "🔄" }
                                p { style: "margin: 0.5rem 0 0; font-size: 0.7rem;", "Easy Returns" }
                            }
                        }
                    }
                }

                // PDP Tabs Section
                div { class: "kh-pdp-tabs",
                    for (tab, label) in [("description", "Description"), ("vendor", "Vendor Info"), ("reviews", "Reviews (24)")] {
                        button {
                            class: if active_tab() == tab { "kh-pdp-tab-btn active" } else { "kh-pdp-tab-btn" },
                            onclick: move |_| active_tab.set(tab),
                            "{label}"
                        }
                    }
                }

                div { class: "kh-pdp-tab-content kh-surface-glass", style: "border: 1px solid var(--color-border-subtle); border-top: none; padding: 2rem; border-bottom-left-radius: var(--radius-xl); border-bottom-right-radius: var(--radius-xl);",
                    match active_tab() {
                        "description" => rsx! {
                            div {
                                p { "Detailed specifications for {p.title}. This product is known for its durability and authentic craftsmanship." }
                                ul { style: "margin-top: 1rem; padding-left: 1.5rem;",
                                    li { "Sourced locally from {p.vendor}" }
                                    li { "High-grade materials" }
                                    li { "Eco-friendly packaging" }
                                    li { "Authentic Liberian quality" }
                                }
                            }
                        },
                        "vendor" => rsx! {
                            div { style: "display: flex; gap: 2rem; align-items: start;",
                                img { src: "https://api.dicebear.com/7.x/avataaars/svg?seed=vendor", style: "width: 4rem; height: 4rem; border-radius: var(--radius-full);" }
                                div {
                                    h3 { style: "margin: 0;", "{p.vendor}" }
                                    p { style: "color: var(--color-brand); font-weight: bold; font-size: var(--text-sm);", "TOP RATED VENDOR" }
                                    p { style: "margin-top: 0.5rem;", "Specializing in {p.category} products since 2024. Providing consistent quality to the Monrovia community." }
                                }
                            }
                        },
                        "reviews" => rsx! {
                            div { style: "display: grid; gap: 1.5rem;",
                                for i in 1..=3 {
                                    div { style: "padding: 1.5rem; background: var(--color-surface); border-radius: var(--radius-lg);",
                                        div { style: "display: flex; justify-content: space-between; margin-bottom: 0.5rem;",
                                            span { style: "font-weight: bold;", "Customer {i}" }
                                            span { style: "color: #fbbf24;", "★★★★★" }
                                        }
                                        p { style: "font-size: var(--text-sm);", "Amazing product! The quality is even better than shown in the pictures. Delivery was super fast." }
                                    }
                                }
                            }
                        },
                        _ => rsx! { div { "Content not found" } }
                    }
                }

                // Related Products
                div { class: "kh-related-section",
                    h2 { class: "kh-related-title", "You might also like" }
                    div { class: "kh-product-grid",
                        for rp in MOCK_PRODUCTS.iter().filter(|rp| rp.category == p.category && rp.id != p.id).take(4) {
                            crate::pages::browse::ProductCard { product: rp.clone() }
                        }
                    }
                }
            }
        }
    }
}
