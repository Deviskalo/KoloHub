use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct OrderRow {
    id: &'static str,
    customer: &'static str,
    date: &'static str,
    total: &'static str,
    status: &'static str,
    items: usize,
}

#[component]
pub fn DashboardOrders() -> Element {
    let mut orders = use_signal(|| {
        vec![
            OrderRow {
                id: "#KH-234",
                customer: "John Doe",
                date: "May 12, 2026",
                total: "LRD 4,500",
                status: "Preparing",
                items: 2,
            },
            OrderRow {
                id: "#KH-233",
                customer: "Marie Johnson",
                date: "May 11, 2026",
                total: "LRD 1,200",
                status: "Shipped",
                items: 1,
            },
            OrderRow {
                id: "#KH-232",
                customer: "Prince Kollie",
                date: "May 10, 2026",
                total: "LRD 8,100",
                status: "Delivered",
                items: 3,
            },
            OrderRow {
                id: "#KH-231",
                customer: "Satta Sirleaf",
                date: "May 09, 2026",
                total: "LRD 2,400",
                status: "Cancelled",
                items: 1,
            },
        ]
    });
    let mut active_tab = use_signal(|| "All Orders".to_string());
    let selected_tab = active_tab();
    let visible_orders = orders()
        .into_iter()
        .filter(|order| selected_tab == "All Orders" || order.status == selected_tab)
        .collect::<Vec<_>>();
    let no_orders = visible_orders.is_empty();

    rsx! {
        div { class: "kh-animate-fade-in-up",
            div { class: "kh-dashboard-header",
                style: "display: flex; justify-content: space-between; align-items: center;",
                div {
                    h1 { class: "kh-dashboard-title", "Order Management" }
                    p { style: "color: var(--color-text-secondary); margin: 0.25rem 0 0;", "Track and fulfill your customer orders." }
                }
                div { style: "display: flex; gap: 0.75rem;",
                    button { class: "kh-btn-outline", style: "padding: 0.75rem 1rem;", "Export CSV" }
                    button { class: "kh-btn-brand", style: "padding: 0.75rem 1.5rem;", "Print Manifest" }
                }
            }

            // Tabs for filtering orders
            div { class: "kh-category-strip", style: "margin-bottom: 2rem;",
                for tab in ["All Orders", "Pending", "Preparing", "Shipped", "Delivered", "Cancelled"] {
                    button {
                        class: if selected_tab == tab { "kh-category-chip active" } else { "kh-category-chip" },
                        onclick: move |_| active_tab.set(tab.to_string()),
                        "{tab}"
                    }
                }
            }

            div { class: "kh-data-table-wrapper",
                table { class: "kh-data-table",
                    thead {
                        tr {
                            th { "Order ID" }
                            th { "Customer" }
                            th { "Date" }
                            th { "Total" }
                            th { "Status" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        for o in visible_orders {
                            tr {
                                td {
                                    div {
                                        p { style: "margin: 0; font-weight: bold; color: var(--color-brand);", "{o.id}" }
                                        p { style: "margin: 0.15rem 0 0; font-size: 0.7rem; color: var(--color-text-muted);", "{o.items} items" }
                                    }
                                }
                                td { "{o.customer}" }
                                td { "{o.date}" }
                                td {
                                    p { style: "margin: 0; font-weight: var(--font-weight-bold);", "{o.total}" }
                                }
                                td {
                                    {
                                        let badge_class = match o.status {
                                            "Pending" => "warning",
                                            "Preparing" => "warning",
                                            "Shipped" => "success",
                                            "Delivered" => "success",
                                            "Cancelled" => "error",
                                            _ => ""
                                        };
                                        rsx! {
                                            span {
                                                class: "kh-badge-status {badge_class}",
                                                "{o.status}"
                                            }
                                        }
                                    }
                                }
                                td {
                                    div { class: "kh-table-actions",
                                        button { class: "kh-btn-outline", style: "padding: 0.5rem; min-width: auto; height: auto;", "👁️" }
                                        button {
                                            class: "kh-btn-outline",
                                            style: "padding: 0.5rem; min-width: auto; height: auto;",
                                            title: "Advance order status",
                                            disabled: o.status == "Delivered" || o.status == "Cancelled",
                                            onclick: move |_| {
                                                let next_status = match o.status {
                                                    "Pending" => "Preparing",
                                                    "Preparing" => "Shipped",
                                                    "Shipped" => "Delivered",
                                                    _ => o.status,
                                                };
                                                if next_status != o.status {
                                                    if let Some(order) = orders.write().iter_mut().find(|order| order.id == o.id) {
                                                        order.status = next_status;
                                                    }
                                                }
                                            },
                                            "📦"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if no_orders {
                p { style: "padding: 2rem; text-align: center; color: var(--color-text-muted);", "No orders match this status." }
            }
        }
    }
}
