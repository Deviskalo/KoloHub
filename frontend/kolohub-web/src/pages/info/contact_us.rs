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
                    div { style: "display: grid; gap: 0.5rem;",
                        crate::components::ui::FormGroup { 
                            label: "Your Name", 
                            crate::components::ui::Input { placeholder: "e.g. John Flomo" }
                        }
                        crate::components::ui::FormGroup { 
                            label: "Phone Number", 
                            crate::components::ui::Input { input_type: "tel", placeholder: "+231 ..." }
                        }
                        crate::components::ui::FormGroup { 
                            label: "Subject", 
                            crate::components::ui::Select { 
                                options: vec![
                                    "Order Issue".to_string(), 
                                    "Vendor Inquiry".to_string(), 
                                    "Payment Support".to_string(), 
                                    "Other".to_string()
                                ]
                            }
                        }
                        crate::components::ui::FormGroup { 
                            label: "Message", 
                            crate::components::ui::TextArea { placeholder: "How can we help you today?" }
                        }
                        div { style: "margin-top: 1rem;",
                            crate::components::ui::Button { "Submit Request" }
                        }
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
