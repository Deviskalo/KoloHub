use crate::router::Route;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct ProductRow {
    id: &'static str,
    name: &'static str,
    price: f64,
    stock: usize,
    status: &'static str,
    image: &'static str,
}

#[component]
pub fn DashboardProducts() -> Element {
    let mock_products = vec![
        ProductRow {
            id: "P-001",
            name: "Organic Tomatoes",
            price: 4.50,
            stock: 120,
            status: "Active",
            image: "https://images.unsplash.com/photo-1592924357228-91a4daadcfea?auto=format&fit=crop&q=80&w=100",
        },
        ProductRow {
            id: "P-002",
            name: "Roasted Coffee",
            price: 8.00,
            stock: 45,
            status: "Active",
            image: "https://images.unsplash.com/photo-1559056199-641a0ac8b55e?auto=format&fit=crop&q=80&w=100",
        },
        ProductRow {
            id: "P-003",
            name: "Handmade Soap",
            price: 12.00,
            stock: 0,
            status: "Out of Stock",
            image: "https://images.unsplash.com/photo-1600857062241-98e5dba7f214?auto=format&fit=crop&q=80&w=100",
        },
        ProductRow {
            id: "P-004",
            name: "Smart Watch",
            price: 120.00,
            stock: 5,
            status: "Low Stock",
            image: "https://images.unsplash.com/photo-1544117518-30dd5ff7a986?auto=format&fit=crop&q=80&w=100",
        },
    ];

    rsx! {
        div { class: "kh-animate-fade-in-up",
            div { class: "kh-dashboard-header",
                style: "display: flex; justify-content: space-between; align-items: center;",
                div {
                    h1 { class: "kh-dashboard-title", "Products Management" }
                    p { style: "color: var(--color-text-secondary); margin: 0.25rem 0 0;", "Manage your inventory, pricing, and stock status." }
                }
                Link { to: Route::DashboardProductAdd {},
                    button {
                        class: "kh-btn-brand",
                        style: "padding: 0.75rem 1.5rem;",
                        "+ Add Product"
                    }
                }
            }

            // Filtering Bar
            div { 
                style: "margin-bottom: 2rem; display: flex; gap: 1rem; flex-wrap: wrap;",
                input { 
                    class: "kh-input", 
                    style: "flex: 1; min-width: 300px; padding: 0.75rem 1rem;",
                    placeholder: "Search products by name or SKU..." 
                }
                select { 
                    class: "kh-input", 
                    style: "width: auto; padding-right: 2.5rem;",
                    option { "All Categories" }
                    option { "Produce" }
                    option { "Electronics" }
                }
                select { 
                    class: "kh-input", 
                    style: "width: auto; padding-right: 2.5rem;",
                    option { "All Status" }
                    option { "Active" }
                    option { "Out of Stock" }
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
                                    div { style: "display: flex; align-items: center; gap: 1.25rem;",
                                        img { 
                                            src: "{p.image}", 
                                            style: "width: 3.5rem; height: 3.5rem; background: var(--color-surface); border-radius: var(--radius-lg); object-fit: cover; border: 1px solid var(--color-border-subtle);" 
                                        }
                                        div {
                                            p { style: "margin: 0; font-weight: var(--font-weight-bold); color: var(--color-text-primary);", "{p.name}" }
                                            p { style: "margin: 0.15rem 0 0; font-size: 0.7rem; color: var(--color-text-muted);", "SKU: {p.id}" }
                                        }
                                    }
                                }
                                td { 
                                    p { style: "margin: 0; font-weight: var(--font-weight-semibold);", "LRD {p.price:.2}" }
                                }
                                td { 
                                    div { style: "display: flex; align-items: center; gap: 0.5rem;",
                                        p { style: "margin: 0;", "{p.stock}" }
                                        if p.stock < 10 && p.stock > 0 {
                                            span { style: "color: var(--color-accent); font-size: 0.7rem;", "⚠️" }
                                        }
                                    }
                                }
                                td {
                                    {
                                        let badge_class = match p.status {
                                            "Active" => "success",
                                            "Out of Stock" => "error",
                                            "Low Stock" => "warning",
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
                                        Link { to: Route::DashboardProductEdit { id: p.id.to_string() },
                                            button { class: "kh-btn-outline", style: "padding: 0.5rem; min-width: auto; height: auto;", "✏️" }
                                        }
                                        button { class: "kh-btn-outline", style: "padding: 0.5rem; min-width: auto; height: auto; color: #ef4444; border-color: #fecaca;", "🗑️" }
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
