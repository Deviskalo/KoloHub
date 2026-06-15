use dioxus::prelude::*;

#[component]
pub fn VendorOverview() -> Element {
    rsx! {
        div { class: "kh-animate-fade-in-up",
            div { class: "kh-dashboard-header",
                div { style: "display: flex; justify-content: space-between; align-items: center;",
                    div {
                        h1 { class: "kh-dashboard-title", "Vendor Dashboard" }
                        p { style: "color: var(--color-text-secondary); margin: 0.25rem 0 0;", "Welcome back, Liberia's Finest! Here's your store's performance." }
                    }
                    div { class: "kh-stat-trend up", style: "font-size: var(--text-sm); padding: 0.5rem 1rem;", 
                        span { style: "margin-right: 0.5rem;", "●" }
                        "Store Live" 
                    }
                }
            }

            // Stat Cards
            div { class: "kh-stats-grid",
                div { class: "kh-stat-card",
                    div { style: "display: flex; justify-content: space-between;",
                        p { class: "kh-stat-label", "Total Revenue" }
                        span { style: "font-size: 1.25rem;", "💰" }
                    }
                    p { class: "kh-stat-val", "LRD 145,200" }
                    span { class: "kh-stat-trend up", "↑ 12.5% vs last month" }
                }
                div { class: "kh-stat-card",
                    div { style: "display: flex; justify-content: space-between;",
                        p { class: "kh-stat-label", "Active Orders" }
                        span { style: "font-size: 1.25rem;", "📦" }
                    }
                    p { class: "kh-stat-val", "58" }
                    span { class: "kh-stat-trend up", "↑ 4 new today" }
                }
                div { class: "kh-stat-card",
                    div { style: "display: flex; justify-content: space-between;",
                        p { class: "kh-stat-label", "Customer Rating" }
                        span { style: "font-size: 1.25rem;", "⭐" }
                    }
                    p { class: "kh-stat-val", "4.8/5.0" }
                    span { class: "kh-stat-trend up", "↑ 0.2 improved" }
                }
                div { class: "kh-stat-card",
                    div { style: "display: flex; justify-content: space-between;",
                        p { class: "kh-stat-label", "Store Visitors" }
                        span { style: "font-size: 1.25rem;", "👥" }
                    }
                    p { class: "kh-stat-val", "1,240" }
                    span { class: "kh-stat-trend down", "↓ 2.4% slow week" }
                }
            }

            // Charts and Activity
            div { class: "kh-dash-row",
                // Chart area
                div { class: "kh-dash-card",
                    div { style: "display: flex; justify-content: space-between; align-items: center; padding-right: 1.5rem;",
                        h2 { class: "kh-dash-card-header", style: "border-bottom: none;", "Sales Analytics" }
                        select { 
                            style: "padding: 0.4rem; border-radius: var(--radius-md); border: 1px solid var(--color-border-subtle); background: var(--color-surface); font-size: 0.75rem;",
                            option { "Last 7 Days" }
                            option { "Last 30 Days" }
                            option { "All Time" }
                        }
                    }
                    div { class: "kh-dash-chart-placeholder",
                        div { style: "text-align: center;",
                            span { style: "font-size: 3rem; display: block; margin-bottom: 1rem; opacity: 0.5;", "📈" }
                            p { "Analytics Engine Loading..." }
                            p { style: "font-size: 0.75rem; color: var(--color-text-muted);", "Real-time sales tracking is active" }
                        }
                    }
                }

                // Recent Activity
                div { class: "kh-dash-card",
                    h2 { class: "kh-dash-card-header", "Recent Orders" }
                    div { class: "kh-activity-list",
                        for (id, price, status, tag) in [
                            ("#KH-234", "LRD 4,500", "Preparing", "kh-stat-trend up"),
                            ("#KH-233", "LRD 1,200", "Dispatched", "kh-stat-trend up"),
                            ("#KH-232", "LRD 8,100", "Delivered", "kh-stat-trend"),
                            ("#KH-231", "LRD 2,400", "Cancelled", "kh-stat-trend down"),
                        ] {
                            div { class: "kh-activity-item",
                                div { class: "kh-activity-info",
                                    p { class: "kh-activity-title", "{id}" }
                                    div { style: "display: flex; align-items: center; gap: 0.5rem;",
                                        p { class: "kh-activity-sub", "2 items" }
                                        span { class: "{tag}", style: "font-size: 0.6rem; padding: 0.1rem 0.4rem;", "{status}" }
                                    }
                                }
                                p { style: "font-weight: var(--font-weight-bold); color: var(--color-text-primary);", "{price}" }
                            }
                        }
                    }
                    div { style: "padding: 1.5rem; text-align: center; border-top: 1px solid var(--color-border-subtle);",
                        crate::components::ui::Button { variant: "outline", style_attr: "width: 100%;", "View All Orders" }
                    }
                }
            }
        }
    }
}
