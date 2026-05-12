use dioxus::prelude::*;

#[component]
pub fn HelpCenter() -> Element {
    rsx! {
        div { class: "kh-container kh-section",
            style: "max-width: 900px; margin: 0 auto; padding-top: 8rem; padding-bottom: 6rem;",
            
            div { class: "kh-text-center", style: "margin-bottom: 4rem;",
                h1 { class: "kh-text-gradient", style: "font-size: 3.5rem; margin-bottom: 1rem;", "Help Center" }
                p { style: "color: var(--color-text-secondary); font-size: 1.1rem;", "Find answers to your questions about shopping and selling on KoloHub." }
            }

            div { style: "display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 1.5rem; margin-bottom: 4rem;",
                div { class: "kh-card kh-surface-glass kh-text-center", style: "padding: 1.5rem;",
                    h4 { "🛍️ Buying" }
                    p { style: "font-size: 0.85rem; color: var(--color-text-secondary);", "Orders, payments, and tracking" }
                }
                div { class: "kh-card kh-surface-glass kh-text-center", style: "padding: 1.5rem;",
                    h4 { "🏪 Selling" }
                    p { style: "font-size: 0.85rem; color: var(--color-text-secondary);", "Store management and payouts" }
                }
                div { class: "kh-card kh-surface-glass kh-text-center", style: "padding: 1.5rem;",
                    h4 { "🔒 Security" }
                    p { style: "font-size: 0.85rem; color: var(--color-text-secondary);", "Account safety and privacy" }
                }
            }

            div { class: "kh-card kh-surface-glass", style: "padding: 3rem;",
                div { class: "kh-prose",
                    h2 { "Frequently Asked Questions" }
                    
                    div { style: "margin-top: 2rem;",
                        h3 { "How do I pay for my order?" }
                        p { "KoloHub supports several payment methods for your convenience:" }
                        ul {
                            li { b { "Mobile Money:" } " You can pay directly via MTN Mobile Money or Orange Money at checkout." }
                            li { b { "Cash on Delivery:" } " Pay our rider in cash when your package arrives. (Available for orders under $200 USD)." }
                            li { b { "Direct Transfer:" } " For large corporate orders, contact us for bank details." }
                        }
                    }

                    hr { style: "margin: 2rem 0; opacity: 0.1;" }
                    
                    h3 { "Where is my order?" }
                    p { "Once your order is confirmed, you will receive a tracking ID. Go to the 'Track Order' link in the footer to see exactly where your package is and which rider is assigned to you." }

                    hr { style: "margin: 2rem 0; opacity: 0.1;" }

                    h3 { "Can I change my delivery address?" }
                    p { "Address changes are only possible before the vendor has dispatched the item. Please contact our support team immediately if you need to update your location." }

                    hr { style: "margin: 2rem 0; opacity: 0.1;" }

                    h3 { "What happens if an item is out of stock?" }
                    p { "If a vendor cannot fulfill your order, the transaction will be canceled immediately, and any Mobile Money payment will be refunded to your account within 24 hours." }

                    hr { style: "margin: 2rem 0; opacity: 0.1;" }

                    h3 { "Do you deliver to the Leeward counties?" }
                    p { "Currently, our direct delivery fleet covers Montserrado and Margibi. For other counties, we utilize partner transport services (buses) to reach major county capitals. Contact us for specific shipping rates." }
                }
            }

            div { class: "kh-text-center", style: "margin-top: 4rem;",
                p { "Still have questions?" }
                a { 
                    href: "/contact", 
                    class: "kh-btn-brand", 
                    style: "margin-top: 1rem;",
                    "Contact Support"
                }
            }
        }
    }
}
