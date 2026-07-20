use crate::router::Route;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct ProductRow {
    id: &'static str,
    name: &'static str,
    category: &'static str,
    price: f64,
    stock: usize,
    status: &'static str,
    image: &'static str,
}

fn initial_products() -> Vec<ProductRow> {
    vec![
        ProductRow { id: "P-001", name: "Organic Tomatoes", category: "Produce", price: 4.50, stock: 120, status: "Active", image: "https://images.unsplash.com/photo-1592924357228-91a4daadcfea?auto=format&fit=crop&q=80&w=100" },
        ProductRow { id: "P-002", name: "Roasted Coffee", category: "Groceries", price: 8.00, stock: 45, status: "Active", image: "https://images.unsplash.com/photo-1559056199-641a0ac8b55e?auto=format&fit=crop&q=80&w=100" },
        ProductRow { id: "P-003", name: "Handmade Soap", category: "Health & Beauty", price: 12.00, stock: 0, status: "Out of Stock", image: "https://images.unsplash.com/photo-1600857062241-98e5dba7f214?auto=format&fit=crop&q=80&w=100" },
        ProductRow { id: "P-004", name: "Smart Watch", category: "Electronics", price: 120.00, stock: 5, status: "Low Stock", image: "https://images.unsplash.com/photo-1544117518-30dd5ff7a986?auto=format&fit=crop&q=80&w=100" },
    ]
}

#[component]
pub fn DashboardProducts() -> Element {
    let mut products = use_signal(initial_products);
    let mut search_query = use_signal(String::new);
    let mut selected_category = use_signal(|| "All Categories".to_string());
    let mut selected_status = use_signal(|| "All Status".to_string());
    let mut pending_delete = use_signal(|| None::<&'static str>);

    let query = search_query().to_lowercase();
    let category = selected_category();
    let status = selected_status();
    let visible_products = products()
        .into_iter()
        .filter(|product| {
            (query.is_empty()
                || product.name.to_lowercase().contains(&query)
                || product.id.to_lowercase().contains(&query))
                && (category == "All Categories" || product.category == category)
                && (status == "All Status" || product.status == status)
        })
        .collect::<Vec<_>>();
    let no_products = visible_products.is_empty();

    rsx! {
        div { class: "kh-animate-fade-in-up",
            div { class: "kh-dashboard-header", style: "display: flex; justify-content: space-between; align-items: center;",
                div {
                    h1 { class: "kh-dashboard-title", "Products Management" }
                    p { style: "color: var(--color-text-secondary); margin: 0.25rem 0 0;", "Manage your inventory, pricing, and stock status." }
                }
                Link { to: Route::DashboardProductAdd {}, button { class: "kh-btn-brand", style: "padding: 0.75rem 1.5rem;", "+ Add Product" } }
            }

            if let Some(product_id) = pending_delete() {
                div { style: "margin-bottom: 1rem; padding: 1rem; border: 1px solid #fecaca; border-radius: var(--radius-md); background: #fff1f2; display: flex; justify-content: space-between; align-items: center; gap: 1rem;",
                    span { "Remove product {product_id}? This only changes the current dashboard session." }
                    div { style: "display: flex; gap: 0.5rem;",
                        button { class: "kh-btn-outline", onclick: move |_| pending_delete.set(None), "Cancel" }
                        button {
                            class: "kh-btn-brand",
                            style: "background: #dc2626; border-color: #dc2626;",
                            onclick: move |_| {
                                products.write().retain(|product| product.id != product_id);
                                pending_delete.set(None);
                            },
                            "Remove"
                        }
                    }
                }
            }

            div { style: "margin-bottom: 2rem; display: flex; gap: 1rem; flex-wrap: wrap;",
                input { class: "kh-input", style: "flex: 1; min-width: 300px; padding: 0.75rem 1rem;", placeholder: "Search products by name or SKU...", value: "{search_query}", oninput: move |event| search_query.set(event.value()) }
                select { class: "kh-input", style: "width: auto; padding-right: 2.5rem;", value: "{selected_category}", onchange: move |event| selected_category.set(event.value()),
                    option { "All Categories" } option { "Produce" } option { "Groceries" } option { "Electronics" } option { "Health & Beauty" }
                }
                select { class: "kh-input", style: "width: auto; padding-right: 2.5rem;", value: "{selected_status}", onchange: move |event| selected_status.set(event.value()),
                    option { "All Status" } option { "Active" } option { "Low Stock" } option { "Out of Stock" }
                }
            }

            p { style: "margin: -1rem 0 1rem; color: var(--color-text-muted); font-size: var(--text-sm);", "{visible_products.len()} products shown" }
            div { class: "kh-data-table-wrapper",
                table { class: "kh-data-table",
                    thead { tr { th { "Product Name" } th { "Price" } th { "Stock" } th { "Status" } th { "Actions" } } }
                    tbody {
                        for product in visible_products {
                            tr { key: "{product.id}",
                                td { div { style: "display: flex; align-items: center; gap: 1.25rem;", img { src: "{product.image}", alt: "{product.name}", style: "width: 3.5rem; height: 3.5rem; background: var(--color-surface); border-radius: var(--radius-lg); object-fit: cover; border: 1px solid var(--color-border-subtle);" } div { p { style: "margin: 0; font-weight: var(--font-weight-bold); color: var(--color-text-primary);", "{product.name}" } p { style: "margin: 0.15rem 0 0; font-size: 0.7rem; color: var(--color-text-muted);", "{product.category} · SKU: {product.id}" } } } }
                                td { p { style: "margin: 0; font-weight: var(--font-weight-semibold);", "LRD {product.price:.2}" } }
                                td { div { style: "display: flex; align-items: center; gap: 0.5rem;", p { style: "margin: 0;", "{product.stock}" } if product.stock < 10 && product.stock > 0 { span { style: "color: var(--color-accent); font-size: 0.7rem;", "⚠️" } } } }
                                td {
                                    {
                                        let badge_class = match product.status {
                                            "Active" => "success",
                                            "Out of Stock" => "error",
                                            _ => "warning",
                                        };
                                        rsx! { span { class: "kh-badge-status {badge_class}", "{product.status}" } }
                                    }
                                }
                                td { div { class: "kh-table-actions", Link { to: Route::DashboardProductEdit { id: product.id.to_string() }, button { class: "kh-btn-outline", style: "padding: 0.5rem; min-width: auto; height: auto;", "✏️" } } button { class: "kh-btn-outline", style: "padding: 0.5rem; min-width: auto; height: auto; color: #ef4444; border-color: #fecaca;", onclick: move |_| pending_delete.set(Some(product.id)), "🗑️" } } }
                            }
                        }
                    }
                }
            }
            if no_products { p { style: "padding: 2rem; text-align: center; color: var(--color-text-muted);", "No products match these filters." } }
        }
    }
}
