use crate::router::Route;
use dioxus::prelude::*;

#[component]
pub fn Checkout() -> Element {
    let mut payment_method = use_signal(|| "mobile_money".to_string());
    let mut order_placed = use_signal(|| false);

    // Mock totals
    let subtotal = 17.00;
    let delivery_fee = 5.00;
    let total = subtotal + delivery_fee;

    if order_placed() {
        return rsx! {
            main { class: "kh-main-shell",
                div { class: "kh-animate-fade-in-up", style: "text-align: center; padding: 6rem 2rem;",
                    span { style: "font-size: 5rem; display: block; margin-bottom: 2rem;", "🎉" }
                    h1 { "Thank you for your order!" }
                    p { style: "color: var(--color-text-secondary); margin-bottom: 3rem;", 
                        "Your order has been placed successfully. We've sent a confirmation to your phone." 
                    }
                    div { style: "display: flex; gap: 1rem; justify-content: center;",
                        Link { to: Route::Home {},
                            crate::components::ui::Button { "Return Home" }
                        }
                        Link { to: Route::Browse { query: "".to_string() },
                            crate::components::ui::Button { variant: "outline", "Continue Shopping" }
                        }
                    }
                }
            }
        };
    }

    rsx! {
        main { class: "kh-main-shell",
            div { class: "kh-animate-fade-in-up",
                h1 { style: "margin-bottom: 2.5rem; font-size: 2.5rem; font-weight: var(--font-weight-bold);", "Secure Checkout" }

                div { class: "kh-checkout-layout",
                    // Main Form Column
                    div { class: "kh-checkout-main",
                        // Delivery Details
                        div { class: "kh-form-section",
                            h2 { class: "kh-form-section-title", 
                                span { "📍" }
                                "Delivery Information" 
                            }
                            div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 1.5rem;",
                                div { class: "kh-form-group",
                                    label { class: "kh-form-label", "First Name" }
                                    input { class: "kh-input", placeholder: "George" }
                                }
                                div { class: "kh-form-group",
                                    label { class: "kh-form-label", "Last Name" }
                                    input { class: "kh-input", placeholder: "Weah" }
                                }
                            }
                            div { class: "kh-form-group",
                                label { class: "kh-form-label", "Phone Number" }
                                input { class: "kh-input", r#type: "tel", placeholder: "077 000 0000" }
                            }
                            div { class: "kh-form-group",
                                label { class: "kh-form-label", "Delivery Address" }
                                input { class: "kh-input", placeholder: "12th Street, Sinkor, Monrovia" }
                            }
                            div { class: "kh-form-group",
                                label { class: "kh-form-label", "Delivery Instructions (Optional)" }
                                textarea { 
                                    class: "kh-input", 
                                    style: "min-height: 80px; resize: vertical; padding: 1rem;",
                                    placeholder: "e.g., Gate is green, call when outside" 
                                }
                            }
                        }

                        // Payment Methods
                        div { class: "kh-form-section",
                            h2 { class: "kh-form-section-title", 
                                span { "💳" }
                                "Payment Method" 
                            }
                            div { class: "kh-payment-options",
                                // Mobile Money
                                div {
                                    class: if payment_method() == "mobile_money" { "kh-payment-opt active" } else { "kh-payment-opt" },
                                    onclick: move |_| payment_method.set("mobile_money".to_string()),
                                    div { class: "kh-payment-opt-circle" }
                                    div {
                                        p { class: "kh-payment-opt-label", "Mobile Money" }
                                        p { class: "kh-payment-opt-desc", "Fast & Secure via MTN or Orange" }
                                    }
                                }
                                // Cash on Delivery
                                div {
                                    class: if payment_method() == "cod" { "kh-payment-opt active" } else { "kh-payment-opt" },
                                    onclick: move |_| payment_method.set("cod".to_string()),
                                    div { class: "kh-payment-opt-circle" }
                                    div {
                                        p { class: "kh-payment-opt-label", "Cash on Delivery" }
                                        p { class: "kh-payment-opt-desc", "Pay when your package arrives" }
                                    }
                                }
                            }
                        }
                    }

                    // Order Summary Column
                    div { class: "kh-summary-card",
                        h2 { style: "margin: 0 0 1.5rem; font-size: 1.25rem;", "Order Summary" }
                        
                        div { style: "margin-bottom: 2rem; display: flex; flex-direction: column; gap: 1rem;",
                            div { class: "kh-summary-row",
                                span { "Items Subtotal" }
                                span { "${subtotal:.2}" }
                            }
                            div { class: "kh-summary-row",
                                span { "Delivery Fee" }
                                span { "${delivery_fee:.2}" }
                            }
                        }

                        div { class: "kh-summary-total",
                            span { "Total" }
                            span { "${total:.2}" }
                        }

                        div { style: "margin-top: 2.5rem; display: flex; flex-direction: column; gap: 1.5rem;",
                            crate::components::ui::Button { 
                                onclick: move |_| order_placed.set(true),
                                "Confirm Order — ${total:.2}" 
                            }
                            p { style: "font-size: 0.75rem; color: var(--color-text-muted); text-align: center; line-height: 1.5;",
                                "By placing your order, you agree to KoloHub's terms of use and privacy policy."
                            }
                        }

                        div { style: "margin-top: 2rem; padding: 1rem; background: var(--color-surface); border-radius: var(--radius-lg); border: 1px solid var(--color-border-subtle);",
                            div { style: "display: flex; align-items: center; gap: 0.75rem; color: var(--color-brand); font-size: 0.8rem; font-weight: bold;",
                                span { "🛡️" }
                                "Secure Transaction"
                            }
                            p { style: "font-size: 0.7rem; color: var(--color-text-muted); margin-top: 0.5rem;",
                                "Your data is encrypted and secure. We never store your mobile money PIN."
                            }
                        }
                    }
                }
            }
        }
    }
}
