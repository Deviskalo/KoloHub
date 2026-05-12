use dioxus::prelude::*;

/// Checkout & payment page — Phase 8.4
#[component]
pub fn Checkout() -> Element {
    let mut payment_method = use_signal(|| "mobile_money".to_string());

    // We bring over the mock totals from the cart
    let total = 14.00; // Mock total

    rsx! {
        main { class: "kh-main-shell",
            div { style: "padding-top: 2rem;",
                h1 {
                    style: "margin: 0 0 1.5rem; font-size: var(--text-2xl); font-weight: var(--font-weight-bold); color: var(--color-text-primary);",
                    "Checkout"
                }

                div { class: "kh-checkout-layout",
                    // Main Form Column
                    div { class: "kh-checkout-main",
                        // Delivery Details
                        div { class: "kh-form-section",
                            h2 { class: "kh-form-section-title", "Delivery Details" }
                            div { class: "kh-form-group",
                                label { class: "kh-form-label", "Full Name" }
                                input { class: "kh-input", placeholder: "e.g., George Weah" }
                            }
                            div { class: "kh-form-group",
                                label { class: "kh-form-label", "Phone Number" }
                                input { class: "kh-input", r#type: "tel", placeholder: "e.g., +231..." }
                            }
                            div { class: "kh-form-group",
                                label { class: "kh-form-label", "Delivery Address" }
                                input { class: "kh-input", placeholder: "e.g., Sinkor, Monrovia" }
                            }
                        }

                        // Payment Methods
                        div { class: "kh-form-section",
                            h2 { class: "kh-form-section-title", "Payment Method" }
                            div { class: "kh-payment-options",
                                // Mobile Money
                                div {
                                    class: "kh-payment-opt",
                                    class: if payment_method() == "mobile_money" { "active" },
                                    onclick: move |_| payment_method.set("mobile_money".to_string()),
                                    div { class: "kh-payment-opt-circle" }
                                    div {
                                        p { class: "kh-payment-opt-label", style: "margin: 0;", "Mobile Money" }
                                        p { class: "kh-payment-opt-desc", style: "margin: 0.15rem 0 0;", "Pay instantly via MTN or Orange." }
                                    }
                                }
                                // Cash on Delivery
                                div {
                                    class: "kh-payment-opt",
                                    class: if payment_method() == "cod" { "active" },
                                    onclick: move |_| payment_method.set("cod".to_string()),
                                    div { class: "kh-payment-opt-circle" }
                                    div {
                                        p { class: "kh-payment-opt-label", style: "margin: 0;", "Cash on Delivery" }
                                        p { class: "kh-payment-opt-desc", style: "margin: 0.15rem 0 0;", "Pay when your package arrives." }
                                    }
                                }
                            }
                        }
                    }

                    // Order Summary Column
                    div { class: "kh-summary-card",
                        h2 { style: "margin: 0 0 1.25rem; font-size: var(--text-lg); font-weight: var(--font-weight-bold);", "Total to Pay" }
                        p { class: "kh-summary-total", style: "border-top: none; padding-top: 0; margin-top: 0;", "${total:.2}" }

                        a {
                            href: "/track",
                            style: "display: flex; align-items: center; justify-content: center; height: 3.5rem; margin-top: 1.5rem; border-radius: var(--radius-md); background: var(--color-brand); color: #fff; font-size: var(--text-base); font-weight: var(--font-weight-bold); text-decoration: none;",
                            "Confirm Order"
                        }

                        p { style: "margin: 1rem 0 0; font-size: var(--text-xs); color: var(--color-text-muted); text-align: center;",
                            "By confirming, you agree to our Terms of Service."
                        }
                    }
                }
            }
        }
    }
}
