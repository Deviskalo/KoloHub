use dioxus::prelude::*;

#[component]
pub fn SellerGuidelines() -> Element {
    rsx! {
        div { class: "kh-container kh-section",
            style: "max-width: 900px; margin: 0 auto; padding-top: 8rem; padding-bottom: 6rem;",
            
            div { class: "kh-text-center", style: "margin-bottom: 4rem;",
                h1 { class: "kh-text-gradient", style: "font-size: 3.5rem; margin-bottom: 1rem;", "Seller Guidelines" }
                p { style: "color: var(--color-text-secondary); font-size: 1.1rem;", "Building a trusted marketplace for the Liberian community." }
            }

            div { class: "kh-card kh-surface-glass", style: "padding: 3rem; margin-bottom: 2rem;",
                div { class: "kh-prose",
                    h2 { "Our Core Mission" }
                    p { "KoloHub is designed to empower local entrepreneurs while providing customers with a seamless shopping experience. To achieve this, every vendor must adhere to these community standards." }
                    
                    hr { style: "margin: 2rem 0; opacity: 0.1;" }

                    h3 { "1. Product Standards & Quality" }
                    div { class: "kh-card kh-surface-muted", style: "padding: 1.5rem; border-left: 4px solid var(--color-brand);",
                        p { "Every item listed must meet our 'Retail-Ready' criteria. This means no damaged packaging, no expired goods, and no items that differ from their description." }
                    }
                    ul {
                        li { b { "Authenticity:" } " Counterfeit goods are strictly prohibited. Listing fakes will result in immediate and permanent account suspension." }
                        li { b { "Inventory Management:" } " Keep your stock levels updated. Canceling more than 5% of orders due to 'Out of Stock' will lower your visibility." }
                    }

                    h3 { "2. Prohibited Categories" }
                    p { "To ensure safety and legal compliance, the following cannot be sold on KoloHub:" }
                    div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin: 1rem 0;",
                        ul {
                            li { "Prescription medications" }
                            li { "Hazardous chemicals" }
                            li { "Used undergarments" }
                        }
                        ul {
                            li { "Stolen or unregistered electronics" }
                            li { "Explosives or weapons" }
                            li { "Alcohol (without valid license)" }
                        }
                    }

                    h3 { "3. Pricing Integrity" }
                    p { "Fair pricing is key to our ecosystem. Vendors found to be price-gouging on essential goods will be flagged." }
                    ul {
                        li { "All prices must include any applicable local taxes." }
                        li { "Hidden fees added after the purchase are strictly forbidden." }
                    }

                    h3 { "4. Performance & Reliability" }
                    div { style: "display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 1.5rem; margin-top: 2rem;",
                        div { class: "kh-text-center",
                            h4 { style: "color: var(--color-brand); margin-bottom: 0.5rem;", "95%+" }
                            p { style: "font-size: 0.85rem;", "Order Fulfillment Rate" }
                        }
                        div { class: "kh-text-center",
                            h4 { style: "color: var(--color-brand); margin-bottom: 0.5rem;", "< 24h" }
                            p { style: "font-size: 0.85rem;", "Package Prep Time" }
                        }
                        div { class: "kh-text-center",
                            h4 { style: "color: var(--color-brand); margin-bottom: 0.5rem;", "4.0+" }
                            p { style: "font-size: 0.85rem;", "Avg Customer Rating" }
                        }
                    }
                }
            }

            div { class: "kh-card", style: "background: var(--color-brand); color: white; padding: 2rem; border-radius: var(--radius-xl);",
                h3 { "Ready to start selling?" }
                p { "Join over 500 local vendors already growing their business on KoloHub." }
                a { 
                    href: "/dashboard", 
                    class: "kh-btn-outline", 
                    style: "margin-top: 1rem; background: white; color: var(--color-brand); border: none;",
                    "Visit Vendor Portal"
                }
            }
        }
    }
}
