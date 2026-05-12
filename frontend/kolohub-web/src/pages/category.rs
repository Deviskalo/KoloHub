use crate::pages::browse::{Product, ProductCard, MOCK_PRODUCTS};
use dioxus::prelude::*;

#[component]
pub fn CategoryPage(name: String) -> Element {
    // Filter products by category (mocked)
    let filtered_products = MOCK_PRODUCTS
        .iter()
        .filter(|p| p.category.eq_ignore_ascii_case(&name))
        .collect::<Vec<_>>();

    rsx! {
        main { class: "kh-main-shell",
            // Category Header
            div {
                class: "kh-surface-glass kh-animate-fade-in-up",
                style: "padding: 4rem 2rem; border-radius: var(--radius-2xl); margin-bottom: 3rem; text-align: center; border: 1px solid var(--color-border-subtle);",
                h1 { style: "font-size: 3rem; font-weight: var(--font-weight-bold); margin: 0;",
                    "{name}"
                }
                p { style: "color: var(--color-text-secondary); margin-top: 1rem; font-size: var(--text-lg);",
                    "Discover the best {name.to_lowercase()} deals in Liberia."
                }
            }

            // Sub-category Filter (Mocked)
            div {
                class: "kh-animate-fade-in-up",
                style: "animation-delay: 0.1s; display: flex; gap: 1rem; margin-bottom: 3rem; overflow-x: auto; padding-bottom: 1rem;",
                for sub in ["All", "New Arrivals", "Best Sellers", "Top Rated"] {
                    button {
                        class: if sub == "All" { "kh-btn-brand" } else { "kh-btn-outline" },
                        style: "white-space: nowrap;",
                        "{sub}"
                    }
                }
            }

            // Product Grid
            if filtered_products.is_empty() {
                div { style: "text-align: center; padding: 5rem 0;",
                    span { style: "font-size: 4rem;", "🔍" }
                    h3 { "No products found in this category." }
                    p { "Try checking other categories or searching for something else." }
                }
            } else {
                div {
                    class: "kh-animate-fade-in-up",
                    style: "animation-delay: 0.2s;",
                    div { class: "kh-product-grid",
                        for p in filtered_products {
                            ProductCard { product: p.clone() }
                        }
                    }
                }
            }

            // Help Strip
            div {
                class: "kh-animate-fade-in-up",
                style: "animation-delay: 0.3s; margin-top: 6rem; padding: 3rem; background: var(--color-surface-elevated); border-radius: var(--radius-xl); display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 2rem;",
                div {
                    h4 { style: "margin: 0;", "Need help choosing?" }
                    p { style: "margin: 0.25rem 0 0; color: var(--color-text-secondary);",
                        "Our support team is available 24/7."
                    }
                }
                crate::components::ui::Button { variant: "outline", onclick: move |_| {}, "Visit Help Center" }
            }
        }
    }
}
