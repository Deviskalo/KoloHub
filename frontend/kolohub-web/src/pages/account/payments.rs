use crate::components::ui::Button;
use dioxus::prelude::*;

#[component]
pub fn AccountPayments() -> Element {
    rsx! {
        div { class: "kh-animate-fade-in-up",
            div { style: "margin-bottom: 2.5rem; display: flex; justify-content: space-between; align-items: flex-end;",
                div {
                    h2 { style: "font-size: 1.75rem; font-weight: var(--font-weight-bold); margin-bottom: 0.5rem;", "Payment Methods" }
                    p { style: "color: var(--color-text-secondary);", "Manage your cards and Liberian Mobile Money accounts." }
                }
                Button { variant: "brand", "+ Add New Method" }
            }

            div { style: "display: grid; grid-template-columns: 1fr; gap: 2.5rem;",

                // Credit/Debit Cards
                div { class: "kh-dash-card",
                    div { class: "kh-dash-card-header", "Credit & Debit Cards" }
                    div { class: "kh-activity-list",
                        div { class: "kh-activity-item",
                            div { style: "display: flex; gap: 1.5rem; align-items: center;",
                                div {
                                    style: "width: 4rem; height: 2.5rem; background: #e0e0e0; border-radius: var(--radius-md); display: flex; align-items: center; justify-content: center; font-weight: bold; font-size: 0.7rem; color: #666; border: 1px solid var(--color-border-subtle);",
                                    "VISA"
                                }
                                div {
                                    p { class: "kh-activity-title", "Visa ending in 4242" }
                                    p { class: "kh-activity-sub", "Expires 12/2026 • Billing Zip: 1000" }
                                }
                                span {
                                    style: "font-size: var(--text-xs); background: var(--color-brand-muted); color: var(--color-brand); padding: 0.2rem 0.5rem; border-radius: var(--radius-sm); font-weight: bold;",
                                    "PRIMARY"
                                }
                            }
                            div { style: "display: flex; gap: 1rem;",
                                Button { variant: "outline", "Edit" }
                                Button { variant: "outline", "Remove" }
                            }
                        }
                    }
                }

                // Mobile Money Section
                div { class: "kh-dash-card",
                    div { class: "kh-dash-card-header", "Liberian Mobile Money" }
                    div { class: "kh-activity-list",
                        div { class: "kh-activity-item",
                            div { style: "display: flex; gap: 1.5rem; align-items: center;",
                                div {
                                    style: "width: 4rem; height: 2.5rem; background: #ff7900; border-radius: var(--radius-md); display: flex; align-items: center; justify-content: center; font-weight: bold; font-size: 0.7rem; color: #fff;",
                                    "Orange"
                                }
                                div {
                                    p { class: "kh-activity-title", "Orange Money Account" }
                                    p { class: "kh-activity-sub", "+231 88 000 0000" }
                                }
                            }
                            Button { variant: "brand", "Link Account" }
                        }
                        div { class: "kh-activity-item",
                            div { style: "display: flex; gap: 1.5rem; align-items: center;",
                                div {
                                    style: "width: 4rem; height: 2.5rem; background: #ffcc00; border-radius: var(--radius-md); display: flex; align-items: center; justify-content: center; font-weight: bold; font-size: 0.7rem; color: #333;",
                                    "MTN"
                                }
                                div {
                                    p { class: "kh-activity-title", "LoneStar MTN Mobile Money" }
                                    p { class: "kh-activity-sub", "+231 77 000 0000" }
                                }
                            }
                            Button { variant: "outline", "Remove" }
                        }
                    }
                }

                // Transaction History Quick Link
                div { style: "text-align: center; padding: 2rem;",
                    p { style: "color: var(--color-text-secondary); margin-bottom: 1rem;", "Looking for your billing history?" }
                    crate::components::ui::Button { variant: "outline", "View All Transactions" }
                }
            }
        }
    }
}
