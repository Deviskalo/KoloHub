use dioxus::prelude::*;

#[component]
pub fn VendorOverview() -> Element {
    rsx! {
        div {
            div { class: "kh-dashboard-header",
                h1 { class: "kh-dashboard-title", "Vendor Overview" }
                p { style: "color: var(--color-text-secondary); margin: 0.25rem 0 0;", "Welcome back. Here is what is happening with your store today." }
            }

            // Stat Cards
            div { class: "kh-stats-grid",
                div { class: "kh-stat-card",
                    p { class: "kh-stat-label", "Total Sales" }
                    p { class: "kh-stat-val", "LRD 145,200" }
                    span { class: "kh-stat-trend up", "↑ 12.5%" }
                }
                div { class: "kh-stat-card",
                    p { class: "kh-stat-label", "Total Orders" }
                    p { class: "kh-stat-val", "58" }
                    span { class: "kh-stat-trend up", "↑ 5.2%" }
                }
                div { class: "kh-stat-card",
                    p { class: "kh-stat-label", "Store Conversion" }
                    p { class: "kh-stat-val", "8.6%" }
                    span { class: "kh-stat-trend down", "↓ 1.1%" }
                }
            }

            // Charts and Activity
            div { class: "kh-dash-row",
                // Chart area
                div { class: "kh-dash-card",
                    h2 { class: "kh-dash-card-header", "Sales Overview" }
                    div { class: "kh-dash-chart-placeholder",
                        "[ Chart Visualization Placeholder ]"
                    }
                }

                // Recent Activity
                div { class: "kh-dash-card",
                    h2 { class: "kh-dash-card-header", "Recent Orders" }
                    div { class: "kh-activity-list",
                        div { class: "kh-activity-item",
                            div { class: "kh-activity-info",
                                p { class: "kh-activity-title", "Order #KH-234" }
                                p { class: "kh-activity-sub", "2 items • Preparing" }
                            }
                            p { style: "font-weight: var(--font-weight-bold);", "LRD 4,500" }
                        }
                        div { class: "kh-activity-item",
                            div { class: "kh-activity-info",
                                p { class: "kh-activity-title", "Order #KH-233" }
                                p { class: "kh-activity-sub", "1 item • Dispatched" }
                            }
                            p { style: "font-weight: var(--font-weight-bold);", "LRD 1,200" }
                        }
                        div { class: "kh-activity-item",
                            div { class: "kh-activity-info",
                                p { class: "kh-activity-title", "Order #KH-232" }
                                p { class: "kh-activity-sub", "3 items • Delivered" }
                            }
                            p { style: "font-weight: var(--font-weight-bold);", "LRD 8,100" }
                        }
                    }
                }
            }
        }
    }
}
