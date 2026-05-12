use crate::pages::browse::MOCK_PRODUCTS;
use crate::router::Route;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct CartItem {
    id: String,
    title: String,
    price: f64,
    quantity: usize,
    image_url: String,
}

#[component]
pub fn Cart() -> Element {
    let mut items = use_signal(|| {
        vec![
            CartItem {
                id: MOCK_PRODUCTS[0].id.to_string(),
                title: MOCK_PRODUCTS[0].title.to_string(),
                price: 4.50,
                quantity: 2,
                image_url: MOCK_PRODUCTS[0].image_url.to_string(),
            },
            CartItem {
                id: MOCK_PRODUCTS[5].id.to_string(),
                title: MOCK_PRODUCTS[5].title.to_string(),
                price: 8.00,
                quantity: 1,
                image_url: MOCK_PRODUCTS[5].image_url.to_string(),
            },
        ]
    });

    let subtotal: f64 = items()
        .iter()
        .map(|item| item.price * item.quantity as f64)
        .sum();
    let delivery_fee = if subtotal > 50.0 { 0.0 } else { 5.0 };
    let total = subtotal + delivery_fee;
    
    let delivery_fee_text = if delivery_fee == 0.0 { 
        "FREE".to_string() 
    } else { 
        format!("${delivery_fee:.2}") 
    };

    rsx! {
        main { class: "kh-main-shell",
            div { class: "kh-animate-fade-in-up",
                div { style: "display: flex; justify-content: space-between; align-items: baseline; margin-bottom: 2rem; border-bottom: 1px solid var(--color-border-subtle); padding-bottom: 1rem;",
                    h1 { style: "margin: 0; font-size: 2.5rem; font-weight: var(--font-weight-bold);", "Shopping Cart" }
                    span { style: "color: var(--color-text-secondary); font-weight: var(--font-weight-semibold);", "{items().len()} Items" }
                }

                if items().is_empty() {
                    div { class: "kh-empty-cart",
                        span { style: "font-size: 4rem; display: block; margin-bottom: 1rem;", "🛒" }
                        h2 { "Your cart is feeling a bit light" }
                        p { style: "color: var(--color-text-secondary); margin-bottom: 2rem;", "Explore our marketplace and discover amazing local products." }
                        Link { to: Route::Browse { query: "".to_string() },
                            crate::components::ui::Button { "Start Shopping" }
                        }
                    }
                } else {
                    div { class: "kh-cart-layout",
                        // Items List
                        div { class: "kh-cart-items",
                            for (index, item) in items().iter().enumerate() {
                                div { class: "kh-cart-item",
                                    img { class: "kh-cart-item-img", src: "{item.image_url}", alt: "{item.title}" }
                                    div { class: "kh-cart-item-info",
                                        Link { to: Route::Product { id: item.id.clone() },
                                            p { class: "kh-cart-item-title", "{item.title}" }
                                        }
                                        p { class: "kh-cart-item-price", "${item.price:.2}" }
                                        
                                        // Mobile controls placeholder
                                        div { style: "display: flex; align-items: center; gap: 1rem; margin-top: 0.5rem;",
                                            div { class: "kh-stepper",
                                                button {
                                                    class: "kh-stepper-btn",
                                                    onclick: move |_| {
                                                        let mut current = items();
                                                        if current[index].quantity > 1 {
                                                            current[index].quantity -= 1;
                                                        } else {
                                                            current.remove(index);
                                                        }
                                                        items.set(current);
                                                    },
                                                    "-"
                                                }
                                                span { class: "kh-stepper-val", "{item.quantity}" }
                                                button {
                                                    class: "kh-stepper-btn",
                                                    onclick: move |_| {
                                                        let mut current = items();
                                                        current[index].quantity += 1;
                                                        items.set(current);
                                                    },
                                                    "+"
                                                }
                                            }
                                            button {
                                                style: "background: none; border: none; color: #ef4444; font-size: 0.8rem; cursor: pointer; text-decoration: underline;",
                                                onclick: move |_| {
                                                    let mut current = items();
                                                    current.remove(index);
                                                    items.set(current);
                                                },
                                                "Remove"
                                            }
                                        }
                                    }
                                    div { style: "text-align: right; font-weight: bold; font-size: var(--text-lg); min-width: 80px;",
                                        "${item.price * item.quantity as f64:.2}"
                                    }
                                }
                            }
                        }

                        // Summary Card
                        div { class: "kh-summary-card",
                            h2 { style: "margin: 0 0 1.5rem; font-size: 1.25rem;", "Order Summary" }
                            
                            div { style: "margin-bottom: 2rem; display: flex; flex-direction: column; gap: 1rem;",
                                div { class: "kh-summary-row",
                                    span { "Subtotal" }
                                    span { "${subtotal:.2}" }
                                }
                                div { class: "kh-summary-row",
                                    span { "Delivery Fee" }
                                    span { "{delivery_fee_text}" }
                                }
                                if subtotal < 50.0 {
                                    p { style: "font-size: 0.75rem; color: var(--color-brand); margin: 0;",
                                        "Add ${50.0 - subtotal:.2} more for FREE delivery!"
                                    }
                                    div { class: "kh-progress-bar",
                                        div { class: "kh-progress-fill", style: "width: {(subtotal / 50.0 * 100.0).min(100.0)}%;" }
                                    }
                                }
                            }

                            div { class: "kh-summary-total",
                                span { "Total" }
                                span { "${total:.2}" }
                            }

                            div { style: "margin-top: 2rem; display: flex; flex-direction: column; gap: 1rem;",
                                Link { to: Route::Checkout {},
                                    crate::components::ui::Button { "Proceed to Checkout" }
                                }
                                Link { to: Route::Browse { query: "".to_string() },
                                    crate::components::ui::Button { variant: "outline", "Continue Shopping" }
                                }
                            }

                            div { style: "margin-top: 1.5rem; display: flex; align-items: center; gap: 0.5rem; color: var(--color-text-muted); font-size: 0.75rem; justify-content: center;",
                                span { "🛡️" }
                                span { "Secure SSL Encryption" }
                            }
                        }
                    }
                }
            }
        }
    }
}
