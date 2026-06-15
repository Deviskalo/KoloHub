use crate::components::ui::Button;
use dioxus::prelude::*;

#[component]
pub fn AccountRewards() -> Element {
    rsx! {
        div { class: "kh-animate-fade-in-up",
            div { style: "margin-bottom: 2.5rem; display: flex; justify-content: space-between; align-items: flex-end;",
                div {
                    h2 { style: "font-size: 1.75rem; font-weight: var(--font-weight-bold); margin-bottom: 0.5rem;", "My Kolo Rewards" }
                    p { style: "color: var(--color-text-secondary);", "Earn points with every local purchase and redeem for exclusive discounts." }
                }
                div { class: "kh-btn-accent", style: "height: 3rem; padding: 0 1.5rem; display: flex; align-items: center; gap: 0.75rem; border-radius: var(--radius-full); font-weight: bold;",
                    span { style: "font-size: 1.25rem;", "💎" }
                    "1,250 Points"
                }
            }

            div { style: "display: grid; grid-template-columns: 1fr; gap: 2.5rem;",

                // Active Coupons Column
                div { class: "kh-dash-card",
                    div { class: "kh-dash-card-header", "Active Coupons" }
                    div { class: "kh-activity-list",
                        for (code, label, desc, exp) in [
                            ("FIRSTBUY", "15% OFF", "Valid on your first harvest order.", "Exp: June 30, 2024"),
                            ("VENDOR10", "$10.00 Credit", "Special vendor-sponsored credit.", "Exp: July 15, 2024"),
                        ] {
                            div { class: "kh-activity-item",
                                div { style: "display: flex; gap: 1.25rem; align-items: center;",
                                    div {
                                        style: "width: 3.5rem; height: 3.5rem; background: var(--color-brand-muted); color: var(--color-brand); border-radius: var(--radius-md); display: flex; align-items: center; justify-content: center; font-weight: bold; font-size: 0.9rem; text-align: center; padding: 0.25rem;",
                                        "{label}"
                                    }
                                    div {
                                        p { class: "kh-activity-title", "Code: {code}" }
                                        p { class: "kh-activity-sub", "{desc}" }
                                    }
                                }
                                div { style: "text-align: right;",
                                    p { style: "margin: 0; font-size: var(--text-xs); color: var(--color-text-muted);", "{exp}" }
                                    Button { variant: "outline", style_attr: "margin-top: 0.5rem; height: 2.2rem; font-size: var(--text-xs);", "Copy Code" }
                                }
                            }
                        }
                    }
                }

                // Points History / Redemption
                div { class: "kh-dash-card",
                    div { class: "kh-dash-card-header", "Earning History" }
                    div { class: "kh-activity-list",
                        for (action, points, date) in [
                            ("Purchase: Fresh Pineapple Bulk", "+120", "Yesterday"),
                            ("Purchase: Lappa Suits (x2)", "+450", "3 days ago"),
                            ("Referral Bonus: Prince K.", "+500", "May 28"),
                            ("Daily Login Bonus", "+10", "May 27"),
                        ] {
                            div { class: "kh-activity-item",
                                div {
                                    p { class: "kh-activity-title", "{action}" }
                                    p { class: "kh-activity-sub", "{date}" }
                                }
                                span {
                                    style: "font-weight: bold; color: var(--color-brand);",
                                    "{points}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
