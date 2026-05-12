use dioxus::prelude::*;

#[component]
pub fn Returns() -> Element {
    rsx! {
        div { class: "kh-container kh-section",
            style: "max-width: 900px; margin: 0 auto; padding-top: 8rem; padding-bottom: 6rem;",
            
            div { class: "kh-text-center", style: "margin-bottom: 4rem;",
                h1 { class: "kh-text-gradient", style: "font-size: 3.5rem; margin-bottom: 1rem;", "Returns & Refunds" }
                p { style: "color: var(--color-text-secondary); font-size: 1.1rem;", "Simple, transparent returns for a worry-free shopping experience." }
            }

            div { class: "kh-card kh-surface-glass", style: "padding: 3rem; margin-bottom: 3rem;",
                div { class: "kh-prose",
                    h2 { "The KoloHub Guarantee" }
                    p { "We stand by every product sold on our platform. If an item is defective or significantly different from its description, you are covered." }
                    
                    div { style: "display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 1.5rem; margin: 3rem 0;",
                        div { class: "kh-card kh-surface-muted kh-text-center", style: "padding: 1.5rem;",
                            h4 { "48 Hours" }
                            p { style: "font-size: 0.8rem;", "Standard Return Window" }
                        }
                        div { class: "kh-card kh-surface-muted kh-text-center", style: "padding: 1.5rem;",
                            h4 { "Free Pickup" }
                            p { style: "font-size: 0.8rem;", "For defective items" }
                        }
                        div { class: "kh-card kh-surface-muted kh-text-center", style: "padding: 1.5rem;",
                            h4 { "24-Hour" }
                            p { style: "font-size: 0.8rem;", "Refund Processing" }
                        }
                    }

                    h3 { "How to Process a Return" }
                    ol {
                        li { b { "Take a Photo:" } " Capture the issue (damage, wrong size, etc.) clearly." }
                        li { b { "Open a Ticket:" } " Go to your 'Orders' page and click 'Return Item' or message us on WhatsApp." }
                        li { b { "Handover:" } " Our rider will visit your address to verify and collect the item." }
                        li { b { "Refund:" } " Once the item is back at our hub, your refund is initiated." }
                    }

                    h3 { "Refund Methods" }
                    p { "Refunds are processed back to your original payment method:" }
                    ul {
                        li { b { "Mobile Money:" } " Credits back to your MTN/Orange wallet within 24 hours." }
                        li { b { "Cash Purchases:" } " Cash refund delivered by rider or KoloHub credit voucher." }
                    }

                    hr { style: "margin: 3rem 0; opacity: 0.1;" }

                    h3 { "Non-Returnable Items" }
                    p { "The following items cannot be returned due to health, safety, and hygiene reasons:" }
                    div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 1rem;",
                        ul {
                            li { "Food and Perishables" }
                            li { "Beauty & Skin Care (opened)" }
                            li { "Underwear & Swimwear" }
                        }
                        ul {
                            li { "Custom-made/Tailored clothing" }
                            li { "Used Bedding/Linens" }
                            li { "Pre-paid vouchers/cards" }
                        }
                    }
                }
            }
        }
    }
}
