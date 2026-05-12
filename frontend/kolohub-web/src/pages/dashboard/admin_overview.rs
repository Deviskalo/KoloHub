use dioxus::prelude::*;

#[component]
pub fn AdminOverview() -> Element {
    rsx! {
        div {
            div { class: "kh-dashboard-header",
                h1 { class: "kh-dashboard-title", "Platform Administration" }
                p { style: "color: var(--color-text-secondary); margin: 0.25rem 0 0;", "Supervise platform activity, vendors, and total revenue." }
            }

            // Stat Cards
            div { class: "kh-stats-grid",
                div { class: "kh-stat-card",
                    p { class: "kh-stat-label", "Total Platform Users" }
                    p { class: "kh-stat-val", "12,540" }
                    span { class: "kh-stat-trend up", "↑ 18.0%" }
                }
                div { class: "kh-stat-card",
                    p { class: "kh-stat-label", "Total Vendors" }
                    p { class: "kh-stat-val", "3,245" }
                    span { class: "kh-stat-trend up", "↑ 12.1%" }
                }
                div { class: "kh-stat-card",
                    p { class: "kh-stat-label", "Global Revenue" }
                    p { class: "kh-stat-val", "LRD 450.2M" }
                    span { class: "kh-stat-trend up", "↑ 9.4%" }
                }
            }

            // Charts and Activity
            div { class: "kh-dash-row",
                // Chart area
                div { class: "kh-dash-card",
                    h2 { class: "kh-dash-card-header", "Platform Revenue Timeline" }
                    div { class: "kh-dash-chart-placeholder",
                        "[ Network Growth Chart Placeholder ]"
                    }
                }

                // Recent Activity / Top Vendors
                div { class: "kh-dash-card",
                    h2 { class: "kh-dash-card-header", "Top Vendors" }
                    div { class: "kh-activity-list",
                        div { class: "kh-activity-item",
                            div { class: "kh-activity-info",
                                p { class: "kh-activity-title", "Monrovia Electronics" }
                                p { class: "kh-activity-sub", "892 sales • 4.8★" }
                            }
                            p { style: "font-weight: var(--font-weight-bold);", "LRD 14.5M" }
                        }
                        div { class: "kh-activity-item",
                            div { class: "kh-activity-info",
                                p { class: "kh-activity-title", "Karmon's Boutique" }
                                p { class: "kh-activity-sub", "651 sales • 4.9★" }
                            }
                            p { style: "font-weight: var(--font-weight-bold);", "LRD 8.2M" }
                        }
                        div { class: "kh-activity-item",
                            div { class: "kh-activity-info",
                                p { class: "kh-activity-title", "Fresh Farm Produce" }
                                p { class: "kh-activity-sub", "1,204 sales • 4.6★" }
                            }
                            p { style: "font-weight: var(--font-weight-bold);", "LRD 4.1M" }
                        }
                    }
                }
            }
        }
    }
}
