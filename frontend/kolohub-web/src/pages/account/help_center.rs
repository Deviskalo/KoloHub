use crate::components::ui::Button;
use dioxus::prelude::*;

#[component]
pub fn AccountHelp() -> Element {
    rsx! {
        div { class: "kh-animate-fade-in-up",
            div { style: "margin-bottom: 2.5rem; text-align: center;",
                h2 { style: "font-size: 2rem; font-weight: var(--font-weight-bold); margin-bottom: 0.5rem;", "How can we help you, Johnathan?" }
                p { style: "color: var(--color-text-secondary);", "Search our knowledge base or reach out to our local team." }
            }

            // Quick Category Grid
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1.5rem; margin-bottom: 4rem;",
                for (icon, title, desc) in [
                    ("📦", "Orders & Delivery", "Track issues, returns, or missing items."),
                    ("💳", "Payments & Billing", "Mobile money issues, receipts, and refunds."),
                    ("🛡️", "Safety & Privacy", "Account security and scam protection."),
                    ("💼", "Selling on KoloHub", "Vendor onboarding and payout help."),
                ] {
                    div { class: "kh-dash-card", style: "padding: 1.5rem; cursor: pointer; transition: transform 0.2s;",
                        span { style: "font-size: 2rem; display: block; margin-bottom: 1rem;", "{icon}" }
                        h4 { style: "margin: 0 0 0.5rem 0; font-weight: var(--font-weight-bold);", "{title}" }
                        p { style: "margin: 0; font-size: var(--text-xs); color: var(--color-text-secondary); line-height: 1.5;", "{desc}" }
                    }
                }
            }

            div { class: "kh-dash-row",
                // FAQ Section
                div { class: "kh-dash-card",
                    div { class: "kh-dash-card-header", "Frequent Questions" }
                    div { class: "kh-activity-list",
                        for q in [
                            "How long does delivery take in Monrovia?",
                            "Can I pay with Orange Money on delivery?",
                            "What is the return policy for fresh produce?",
                            "How do I become a verified vendor?",
                        ] {
                            div { class: "kh-activity-item", style: "cursor: pointer;",
                                p { class: "kh-activity-title", style: "font-weight: var(--font-weight-medium);", "{q}" }
                                span { style: "opacity: 0.5;", "→" }
                            }
                        }
                    }
                    div { style: "padding: 1.5rem; text-align: center;",
                        a { href: "#", style: "color: var(--color-brand); font-weight: bold; font-size: var(--text-sm);", "View Knowledge Base" }
                    }
                }

                // Contact Support Column
                div { style: "display: flex; flex-direction: column; gap: 2rem;",
                    div { class: "kh-dash-card",
                        div { class: "kh-dash-card-header", "Contact Our Team" }
                        div { style: "padding: 2rem; text-align: center;",
                            div {
                                style: "width: 4rem; height: 4rem; background: var(--color-brand-muted); border-radius: 50%; display: flex; align-items: center; justify-content: center; margin: 0 auto 1.5rem; font-size: 1.5rem;",
                                "🎧"
                            }
                            h4 { style: "margin-bottom: 0.5rem;", "Talk to an Agent" }
                            p { style: "color: var(--color-text-secondary); font-size: var(--text-sm); margin-bottom: 1.5rem;",
                                "Our team is available Mon-Sat, 8AM - 6PM GMT."
                            }
                            div { style: "display: grid; gap: 1rem;",
                                Button { variant: "brand", style_attr: "width: 100%;", "Start Live Chat" }
                                Button { variant: "outline", style_attr: "width: 100%;", "Call +231 88 000 0000" }
                            }
                        }
                    }
                }
            }
        }
    }
}
