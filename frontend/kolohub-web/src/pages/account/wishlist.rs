use crate::components::ui::Button;
use crate::pages::browse::{ProductCard, MOCK_PRODUCTS};
use dioxus::prelude::*;

#[component]
pub fn AccountWishlist() -> Element {
    rsx! {
        div { class: "kh-animate-fade-in-up",
            div { style: "margin-bottom: 2.5rem; display: flex; justify-content: space-between; align-items: flex-end;",
                div {
                    h2 { style: "font-size: 1.75rem; font-weight: var(--font-weight-bold); margin-bottom: 0.5rem;", "My Wishlist" }
                    p { style: "color: var(--color-text-secondary);", "Save your favorite Liberian harvests and products for later." }
                }
                div { style: "display: flex; gap: 1rem;",
                    Button { variant: "outline", "Share Wishlist" }
                    Button { variant: "brand", "Add All to Cart" }
                }
            }

            if MOCK_PRODUCTS.is_empty() {
                div { class: "kh-empty-cart",
                    span { style: "font-size: 4rem; display: block; margin-bottom: 1.5rem;", "❤️" }
                    h3 { "Your wishlist is empty" }
                    p { style: "color: var(--color-text-secondary); margin-bottom: 2rem;", "Explore the marketplace and save items you love." }
                    Button {
                        variant: "brand",
                        onclick: move |_| { /* Navigate to browse */ },
                        "Start Shopping"
                    }
                }
            } else {
                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(240px, 1fr)); gap: 2rem;",
                    // Using a subset of mock products to simulate a wishlist
                    for p in MOCK_PRODUCTS.iter().take(3) {
                        div { style: "position: relative;",
                            ProductCard { product: p.clone() }
                            button {
                                style: "position: absolute; top: 1rem; right: 1rem; background: var(--color-surface); border: none; border-radius: 50%; width: 2.5rem; height: 2.5rem; cursor: pointer; display: flex; align-items: center; justify-content: center; box-shadow: var(--shadow-sm); z-index: 10; color: #ef4444;",
                                title: "Remove from Wishlist",
                                "✕"
                            }
                            div {
                                style: "margin-top: 1rem; display: grid; gap: 0.5rem;",
                                Button { variant: "brand", style_attr: "width: 100%;", "Add to Cart" }
                            }
                        }
                    }
                }
            }

            // Price Drop Alerts / Suggestions
            div { style: "margin-top: 5rem; padding-top: 3rem; border-top: 1px solid var(--color-border-subtle);",
                h3 { style: "font-size: 1.25rem; font-weight: var(--font-weight-bold); margin-bottom: 1.5rem;", "Price Drop Alerts" }
                div { class: "kh-dash-card",
                    div { style: "padding: 1.5rem; display: flex; align-items: center; gap: 1rem;",
                        span { style: "font-size: 1.5rem;", "🔔" }
                        div {
                            p { style: "margin: 0; font-weight: bold;", "Natural Honey (500ml) is now 15% off!" }
                            p { style: "margin: 0; color: var(--color-text-secondary); font-size: var(--text-sm);", "Matching your wishlist item." }
                        }
                        div { style: "margin-left: auto;",
                            Button { variant: "outline", "View Deal" }
                        }
                    }
                }
            }
        }
    }
}
