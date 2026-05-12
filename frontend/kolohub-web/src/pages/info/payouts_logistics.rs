use dioxus::prelude::*;

#[component]
pub fn PayoutsLogistics() -> Element {
    rsx! {
        div { class: "kh-container kh-section",
            style: "max-width: 900px; margin: 0 auto; padding-top: 8rem; padding-bottom: 6rem;",
            
            div { class: "kh-text-center", style: "margin-bottom: 4rem;",
                h1 { class: "kh-text-gradient", style: "font-size: 3.5rem; margin-bottom: 1rem;", "Payouts & Logistics" }
                p { style: "color: var(--color-text-secondary); font-size: 1.1rem;", "The backbone of commerce: moving money and goods safely across Liberia." }
            }

            div { class: "kh-card kh-surface-glass", style: "padding: 3rem; margin-bottom: 3rem;",
                div { class: "kh-prose",
                    h2 { "Vendor Payout System" }
                    p { "We understand that cash flow is critical for small businesses. KoloHub provides the most reliable payout infrastructure in the region." }
                    
                    div { class: "kh-card kh-surface-muted", style: "padding: 2rem; margin: 2rem 0;",
                        h4 { "Payout Schedule" }
                        table { style: "width: 100%; border-collapse: collapse; margin-top: 1rem;",
                            thead {
                                tr { style: "border-bottom: 1px solid var(--color-border);",
                                    th { style: "text-align: left; padding: 0.75rem;", "Payment Method" }
                                    th { style: "text-align: left; padding: 0.75rem;", "Frequency" }
                                    th { style: "text-align: left; padding: 0.75rem;", "Min. Threshold" }
                                }
                            }
                            tbody {
                                tr { style: "border-bottom: 1px solid var(--color-border);",
                                    td { style: "padding: 0.75rem;", "Mobile Money (MTN/Orange)" }
                                    td { style: "padding: 0.75rem;", "Every Tuesday" }
                                    td { style: "padding: 0.75rem;", "$10.00 USD" }
                                }
                                tr {
                                    td { style: "padding: 0.75rem;", "Bank Transfer (LBDI/Ecobank)" }
                                    td { style: "padding: 0.75rem;", "1st & 15th of Month" }
                                    td { style: "padding: 0.75rem;", "$100.00 USD" }
                                }
                            }
                        }
                    }

                    h3 { "Platform Commission" }
                    p { "To provide delivery insurance and maintain the platform, KoloHub retains a small percentage of each successful sale:" }
                    ul {
                        li { b { "Physical Goods:" } " 8% per transaction" }
                        li { b { "Digital/Voucher Services:" } " 5% per transaction" }
                    }

                    hr { style: "margin: 3rem 0; opacity: 0.1;" }

                    h2 { "Logistics & Delivery Networks" }
                    p { "KoloHub manages a fleet of verified independent riders and partner logistics firms." }

                    div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 2rem; margin-top: 2rem;",
                        div {
                            h4 { style: "color: var(--color-brand);", "Zone 1: Greater Monrovia" }
                            p { style: "font-size: 0.9rem;", "Covers Sinkor, Central Monrovia, Bushrod Island, and Paynesville. Deliveries completed within 24-48 hours." }
                        }
                        div {
                            h4 { style: "color: var(--color-brand);", "Zone 2: Outside Monrovia" }
                            p { style: "font-size: 0.9rem;", "Covers Robertsfield Highway, Brewerville, and Kakata. Deliveries completed within 48-72 hours." }
                        }
                    }

                    h3 { "Damage & Insurance" }
                    p { "All orders shipped via KoloHub Logistics are insured up to 100% of the product value. If an item is damaged during transit by our riders, KoloHub covers the refund cost to the customer while ensuring the vendor still receives their payout." }
                }
            }
        }
    }
}
