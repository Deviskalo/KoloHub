use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct ProductRow {
    id: &'static str,
    name: &'static str,
    price: f64,
    stock: usize,
    status: &'static str,
}

#[component]
pub fn DashboardProducts() -> Element {
    let mock_products = vec![
        ProductRow {
            id: "P-001",
            name: "Fresh Organic Tomatoes",
            price: 4.50,
            stock: 120,
            status: "Active",
        },
        ProductRow {
            id: "P-002",
            name: "Locally Roasted Coffee",
            price: 8.00,
            stock: 45,
            status: "Active",
        },
        ProductRow {
            id: "P-003",
            name: "Wireless Headphones",
            price: 35.00,
            stock: 0,
            status: "Out of Stock",
        },
        ProductRow {
            id: "P-004",
            name: "Smart Watch Series 9",
            price: 120.00,
            stock: 5,
            status: "Low Stock",
        },
    ];

    rsx! {
        div {
            div { class: "kh-dashboard-header",
                style: "display: flex; justify-content: space-between; align-items: center;",
                div {
                    h1 { class: "kh-dashboard-title", "Products Management" }
                    p { style: "color: var(--color-text-secondary); margin: 0.25rem 0 0;", "Manage your inventory, pricing, and stock status." }
                }
                button {
                    class: "kh-btn-brand",
                    "+ Add New Product"
                }
            }

            div { class: "kh-data-table-wrapper",
                table { class: "kh-data-table",
                    thead {
                        tr {
                            th { "Product Name" }
                            th { "Price" }
                            th { "Stock" }
                            th { "Status" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        for p in mock_products {
                            tr {
                                td {
                                    div { style: "display: flex; align-items: center; gap: 1rem;",
                                        div { style: "width: 3rem; height: 3rem; background: var(--color-surface); border-radius: var(--radius-md); border: 1px solid var(--color-border-subtle);" }
                                        span { style: "font-weight: var(--font-weight-semibold);", "{p.name}" }
                                    }
                                }
                                td { "${p.price:.2}" }
                                td { "{p.stock}" }
                                td {
                                    {
                                        let badge_class = match p.status {
                                            "Active" => "success",
                                            "Out of Stock" | "Low Stock" => "warning",
                                            _ => ""
                                        };
                                        rsx! {
                                            span {
                                                class: "kh-badge-status {badge_class}",
                                                "{p.status}"
                                            }
                                        }
                                    }
                                }
                                td {
                                    div { class: "kh-table-actions",
                                        button { class: "kh-btn-outline", style: "padding: 0.25rem 0.5rem; font-size: var(--text-xs);", "Edit" }
                                        button { class: "kh-btn-outline", style: "padding: 0.25rem 0.5rem; font-size: var(--text-xs); color: #ef4444;", "Delete" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
