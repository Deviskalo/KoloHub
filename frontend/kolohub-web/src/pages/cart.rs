use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct CartItem {
    id: &'static str,
    title: &'static str,
    price: f64,
    quantity: usize,
    image_url: &'static str,
}

/// Shopping cart page — Phase 8.3
#[component]
pub fn Cart() -> Element {
    let mut items = use_signal(|| {
        vec![
            CartItem {
                id: "c-001",
                title: "Fresh Organic Tomatoes",
                price: 4.50,
                quantity: 2,
                image_url: "/images/design-ui.png",
            },
            CartItem {
                id: "c-002",
                title: "Locally Roasted Coffee",
                price: 8.00,
                quantity: 1,
                image_url: "/images/design-ui.png",
            },
        ]
    });

    let subtotal: f64 = items()
        .iter()
        .map(|item| item.price * item.quantity as f64)
        .sum();
    let delivery_fee = 5.00;
    let total = subtotal + delivery_fee;

    rsx! {
        main { class: "kh-main-shell",
            div { style: "padding-top: 2rem;",
                h1 {
                    style: "margin: 0 0 1.5rem; font-size: var(--text-2xl); font-weight: var(--font-weight-bold); color: var(--color-text-primary);",
                    "Your Cart"
                }

                div { class: "kh-cart-layout",
                    // Items List
                    div { class: "kh-cart-items",
                        if items().is_empty() {
                            p { style: "color: var(--color-text-secondary);", "Your cart is empty." }
                        } else {
                            for (index, item) in items().iter().enumerate() {
                                div { class: "kh-cart-item",
                                    img { class: "kh-cart-item-img", src: "{item.image_url}", alt: "{item.title}" }
                                    div { class: "kh-cart-item-info",
                                        p { class: "kh-cart-item-title", "{item.title}" }
                                        p { class: "kh-cart-item-price", "${item.price:.2}" }
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
                                    }
                                }
                            }
                        }
                    }

                    // Summary Card
                    div { class: "kh-summary-card",
                        h2 { style: "margin: 0 0 1.25rem; font-size: var(--text-lg); font-weight: var(--font-weight-bold);", "Order Summary" }
                        div { class: "kh-summary-row",
                            span { "Subtotal" }
                            span { "${subtotal:.2}" }
                        }
                        div { class: "kh-summary-row",
                            span { "Delivery Fee" }
                            span { "${delivery_fee:.2}" }
                        }
                        div { class: "kh-summary-total",
                            span { "Total" }
                            span { "${total:.2}" }
                        }

                        a {
                            href: "/checkout",
                            style: "display: flex; align-items: center; justify-content: center; height: 3rem; margin-top: 1.5rem; border-radius: var(--radius-md); background: var(--color-brand); color: #fff; font-size: var(--text-base); font-weight: var(--font-weight-bold); text-decoration: none;",
                            "Proceed to Checkout"
                        }
                    }
                }
            }
        }
    }
}
