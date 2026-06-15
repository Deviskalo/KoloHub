use crate::components::ui::Button;
use dioxus::prelude::*;

#[component]
pub fn AccountSecurity() -> Element {
    rsx! {
        div { class: "kh-animate-fade-in-up",
            div { style: "margin-bottom: 2.5rem;",
                h2 { style: "font-size: 1.75rem; font-weight: var(--font-weight-bold); margin-bottom: 0.5rem;", "Security & Privacy" }
                p { style: "color: var(--color-text-secondary);", "Protect your account with modern security standards." }
            }

            div { style: "display: grid; grid-template-columns: 1fr; gap: 2.5rem;",

                // Password Change Section
                div { class: "kh-dash-card",
                    div { class: "kh-dash-card-header", "Change Password" }
                    div { style: "padding: 1.5rem;",
                        div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 1.5rem; margin-bottom: 2rem;",
                            div {
                                label { style: "display: block; font-size: var(--text-sm); font-weight: bold; margin-bottom: 0.5rem;", "Current Password" }
                                input { r#type: "password", class: "kh-input", placeholder: "••••••••" }
                            }
                            div {} // Spacer for grid alignment
                            div {
                                label { style: "display: block; font-size: var(--text-sm); font-weight: bold; margin-bottom: 0.5rem;", "New Password" }
                                input { r#type: "password", class: "kh-input", placeholder: "••••••••" }
                            }
                            div {
                                label { style: "display: block; font-size: var(--text-sm); font-weight: bold; margin-bottom: 0.5rem;", "Confirm New Password" }
                                input { r#type: "password", class: "kh-input", placeholder: "••••••••" }
                            }
                        }
                        Button { variant: "brand", "Update Password" }
                    }
                }

                // Two-Factor Authentication
                div { class: "kh-dash-card",
                    div { style: "padding: 2rem; display: flex; justify-content: space-between; align-items: center;",
                        div {
                            h3 { style: "margin: 0 0 0.5rem 0; font-size: 1.1rem; font-weight: var(--font-weight-bold);", "Two-Factor Authentication (2FA)" }
                            p { style: "margin: 0; color: var(--color-text-secondary); font-size: var(--text-sm);",
                                "Add an extra layer of security to your account by requiring a code from your mobile device."
                            }
                        }
                        div { class: "kh-toggle-wrapper", // Assuming CSS for toggles exists or adding it
                            label { class: "kh-switch",
                                input { r#type: "checkbox" }
                                span { class: "kh-slider round" }
                            }
                        }
                    }
                }

                // Active Sessions
                div { class: "kh-dash-card",
                    div { class: "kh-dash-card-header", "Active Sessions" }
                    div { class: "kh-activity-list",
                        div { class: "kh-activity-item",
                            div { style: "display: flex; gap: 1rem; align-items: center;",
                                span { style: "font-size: 1.5rem;", "💻" }
                                div {
                                    p { class: "kh-activity-title", "Chrome on Windows — Monrovia, LR" }
                                    p { class: "kh-activity-sub", "Current Session • IP: 41.203.22.XX" }
                                }
                            }
                            Button { variant: "outline", "Log Out" }
                        }
                        div { class: "kh-activity-item",
                            div { style: "display: flex; gap: 1rem; align-items: center;",
                                span { style: "font-size: 1.5rem;", "📱" }
                                div {
                                    p { class: "kh-activity-title", "iPhone 15 Pro — Sinkor, LR" }
                                    p { class: "kh-activity-sub", "Last active: 2 hours ago" }
                                }
                            }
                            Button { variant: "outline", "Log Out" }
                        }
                    }
                }
            }
        }
    }
}
