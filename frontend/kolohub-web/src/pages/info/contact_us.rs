use dioxus::prelude::*;

#[component]
pub fn ContactUs() -> Element {
    rsx! {
        div { class: "kh-container kh-section",
            style: "max-width: 1000px; margin: 0 auto; padding-top: 8rem; padding-bottom: 6rem;",
            
            div { class: "kh-text-center", style: "margin-bottom: 4rem;",
                h1 { class: "kh-text-gradient", style: "font-size: 3.5rem; margin-bottom: 1rem;", "Contact Us" }
                p { style: "color: var(--color-text-secondary); font-size: 1.1rem;", "We're here to help. Reach out to our team across any of these channels." }
            }

            div { style: "display: grid; grid-template-columns: 1.5fr 1fr; gap: 3rem;",
                // Left Column: Form
                div { class: "kh-card kh-surface-glass", style: "padding: 3rem;",
                    h3 { style: "margin-top: 0; margin-bottom: 2rem;", "Send a Message" }
                    div { style: "display: grid; gap: 1.5rem;",
                        div {
                            label { style: "display: block; font-size: 0.85rem; margin-bottom: 0.5rem; color: var(--color-text-secondary);", "Your Name" }
                            input { 
                                r#type: "text", 
                                placeholder: "e.g. John Flomo",
                                style: "width: 100%; padding: 0.75rem; border-radius: var(--radius-md); border: 1px solid var(--color-border); background: var(--color-surface-muted); color: var(--color-text-primary);"
                            }
                        }
                        div {
                            label { style: "display: block; font-size: 0.85rem; margin-bottom: 0.5rem; color: var(--color-text-secondary);", "Phone Number" }
                            input { 
                                r#type: "tel", 
                                placeholder: "+231 ...",
                                style: "width: 100%; padding: 0.75rem; border-radius: var(--radius-md); border: 1px solid var(--color-border); background: var(--color-surface-muted); color: var(--color-text-primary);"
                            }
                        }
                        div {
                            label { style: "display: block; font-size: 0.85rem; margin-bottom: 0.5rem; color: var(--color-text-secondary);", "Subject" }
                            select { 
                                style: "width: 100%; padding: 0.75rem; border-radius: var(--radius-md); border: 1px solid var(--color-border); background: var(--color-surface-muted); color: var(--color-text-primary);",
                                option { "Order Issue" }
                                option { "Vendor Inquiry" }
                                option { "Payment Support" }
                                option { "Other" }
                            }
                        }
                        div {
                            label { style: "display: block; font-size: 0.85rem; margin-bottom: 0.5rem; color: var(--color-text-secondary);", "Message" }
                            textarea { 
                                rows: "5",
                                placeholder: "How can we help you today?",
                                style: "width: 100%; padding: 0.75rem; border-radius: var(--radius-md); border: 1px solid var(--color-border); background: var(--color-surface-muted); color: var(--color-text-primary);"
                            }
                        }
                        button { class: "kh-btn-brand", style: "width: 100%; height: 3rem;", "Submit Request" }
                    }
                }

                // Right Column: Details
                div {
                    div { class: "kh-card kh-surface-glass", style: "padding: 2rem; margin-bottom: 2rem;",
                        h4 { style: "margin-top: 0; color: var(--color-brand);", "Main HQ" }
                        p { "Tubman Boulevard" }
                        p { "Sinkor, 12th Street" }
                        p { "Monrovia, Liberia" }
                    }

                    div { class: "kh-card kh-surface-glass", style: "padding: 2rem; margin-bottom: 2rem;",
                        h4 { style: "margin-top: 0; color: var(--color-brand);", "Direct Lines" }
                        div { style: "margin-top: 1rem; display: grid; gap: 0.75rem;",
                            div {
                                p { style: "font-size: 0.8rem; color: var(--color-text-secondary);", "Customer Support" }
                                p { "+231 77 000 0000" }
                            }
                            div {
                                p { style: "font-size: 0.8rem; color: var(--color-text-secondary);", "Vendor Hotline" }
                                p { "+231 88 000 0000" }
                            }
                            div {
                                p { style: "font-size: 0.8rem; color: var(--color-text-secondary);", "Email" }
                                p { "support@kolohub.com" }
                            }
                        }
                    }

                    div { class: "kh-card kh-surface-glass", style: "padding: 2rem;",
                        h4 { style: "margin-top: 0; color: var(--color-brand);", "Socials" }
                        div { style: "display: flex; gap: 1rem; margin-top: 1rem;",
                            a { href: "#", style: "color: var(--color-text-primary); text-decoration: none;", "Facebook" }
                            a { href: "#", style: "color: var(--color-text-primary); text-decoration: none;", "Instagram" }
                            a { href: "#", style: "color: var(--color-text-primary); text-decoration: none;", "WhatsApp" }
                        }
                    }
                }
            }
        }
    }
}
