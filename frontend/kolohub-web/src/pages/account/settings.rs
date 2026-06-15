use crate::components::ui::Button;
use dioxus::prelude::*;

#[component]
pub fn AccountSettings() -> Element {
    rsx! {
        div { class: "kh-animate-fade-in-up",
            div { style: "margin-bottom: 2.5rem;",
                h2 { style: "font-size: 1.75rem; font-weight: var(--font-weight-bold); margin-bottom: 0.5rem;", "Account Settings" }
                p { style: "color: var(--color-text-secondary);", "Manage your notifications and app-wide preferences." }
            }

            div { style: "display: grid; grid-template-columns: 1fr; gap: 2.5rem;",

                // Notification Preferences
                div { class: "kh-dash-card",
                    div { class: "kh-dash-card-header", "Notification Preferences" }
                    div { style: "padding: 2rem; display: flex; flex-direction: column; gap: 2rem;",

                        div { class: "kh-notification-group",
                            h4 { style: "margin: 0 0 1rem 0; font-size: var(--text-base);", "Order Updates" }
                            div { style: "display: flex; flex-direction: column; gap: 1rem;",
                                NotificationToggle {
                                    label: "Order Status SMS",
                                    desc: "Get real-time delivery tracking via mobile text.",
                                    enabled: true
                                }
                                NotificationToggle {
                                    label: "Email Receipts",
                                    desc: "Receive digital invoices and order summaries.",
                                    enabled: true
                                }
                            }
                        }

                        div { style: "height: 1px; background: var(--color-border-subtle);" }

                        div { class: "kh-notification-group",
                            h4 { style: "margin: 0 0 1rem 0; font-size: var(--text-base);", "Marketing & Discovery" }
                            div { style: "display: flex; flex-direction: column; gap: 1rem;",
                                NotificationToggle {
                                    label: "Price Drop Alerts",
                                    desc: "Notify me when items in my wishlist go on sale.",
                                    enabled: false
                                }
                                NotificationToggle {
                                    label: "Weekly Harvest Newsletter",
                                    desc: "The best local deals and new vendor spotlights.",
                                    enabled: true
                                }
                            }
                        }
                    }
                }

                // Data & Privacy
                div { class: "kh-dash-card",
                    div { class: "kh-dash-card-header", "Data & Export" }
                    div { style: "padding: 2rem;",
                        p { style: "margin: 0 0 1.5rem 0; color: var(--color-text-secondary); font-size: var(--text-sm); line-height: 1.6;",
                            "You can request a full export of your account data, including order history and saved preferences. This process may take up to 24 hours."
                        }
                        Button { variant: "outline", "Request Data Export" }
                    }
                }
            }
        }
    }
}

#[component]
fn NotificationToggle(label: String, desc: String, enabled: bool) -> Element {
    rsx! {
        div { style: "display: flex; justify-content: space-between; align-items: center;",
            div {
                p { style: "margin: 0; font-weight: var(--font-weight-semibold);", "{label}" }
                p { style: "margin: 0.25rem 0 0; color: var(--color-text-secondary); font-size: var(--text-xs);", "{desc}" }
            }
            label { class: "kh-switch",
                input { r#type: "checkbox", checked: enabled }
                span { class: "kh-slider round" }
            }
        }
    }
}
