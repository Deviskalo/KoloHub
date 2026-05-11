use dioxus::prelude::*;

#[component]
pub fn DashboardHighlights() -> Element {
    rsx! {
        section { style: "max-width: 72rem; margin: 0 auto; padding: 0 1rem 3rem;",
            div { class: "kh-dashboard-grid",
                article { style: "background: var(--color-surface-elevated); border: 1px solid var(--color-border-subtle); border-radius: var(--radius-xl); box-shadow: var(--shadow-card); padding: 1rem;",
                    div { style: "display: flex; align-items: center; justify-content: space-between; gap: 0.75rem; margin-bottom: 0.5rem;",
                        h2 { style: "margin: 0; color: var(--color-text-primary);",
                            "Vendor Dashboard"
                        }
                        span { style: "font-size: var(--text-xs); color: var(--color-accent); font-weight: var(--font-weight-semibold);",
                            "+12.5%"
                        }
                    }
                    p { style: "margin: 0 0 0.9rem 0; color: var(--color-text-secondary);",
                        "Sales, Orders, Customers, and Conversion snapshots."
                    }
                    div { class: "kh-stat-grid",
                        div { class: "kh-stat-chip",
                            p { class: "label", "Total Sales" }
                            p { class: "value", "UGX 2.45M" }
                        }
                        div { class: "kh-stat-chip",
                            p { class: "label", "Total Orders" }
                            p { class: "value", "58" }
                        }
                        div { class: "kh-stat-chip",
                            p { class: "label", "Conversion" }
                            p { class: "value", "8.6%" }
                        }
                    }
                }
                article { style: "background: var(--color-surface-elevated); border: 1px solid var(--color-border-subtle); border-radius: var(--radius-xl); box-shadow: var(--shadow-card); padding: 1rem;",
                    div { style: "display: flex; align-items: center; justify-content: space-between; gap: 0.75rem; margin-bottom: 0.5rem;",
                        h2 { style: "margin: 0; color: var(--color-text-primary);",
                            "Admin Dashboard"
                        }
                        span { style: "font-size: var(--text-xs); color: var(--color-accent); font-weight: var(--font-weight-semibold);",
                            "+18.0%"
                        }
                    }
                    p { style: "margin: 0 0 0.9rem 0; color: var(--color-text-secondary);",
                        "Platform users, vendors, orders, and revenue overview."
                    }
                    div { class: "kh-stat-grid",
                        div { class: "kh-stat-chip",
                            p { class: "label", "Total Users" }
                            p { class: "value", "12,540" }
                        }
                        div { class: "kh-stat-chip",
                            p { class: "label", "Total Vendors" }
                            p { class: "value", "3,245" }
                        }
                        div { class: "kh-stat-chip",
                            p { class: "label", "Revenue" }
                            p { class: "value", "UGX 450.2M" }
                        }
                    }
                }
            }
        }
    }
}
