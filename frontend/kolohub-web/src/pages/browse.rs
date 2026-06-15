use crate::router::Route;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Product {
    pub id: &'static str,
    pub title: &'static str,
    pub vendor: &'static str,
    pub price: &'static str,
    pub image_url: &'static str,
    pub category: &'static str,
}

impl Product {
    pub fn price_val(&self) -> f64 {
        self.price.trim_start_matches('$').replace(',', "").parse().unwrap_or(0.0)
    }
}

pub const MOCK_PRODUCTS: &[Product] = &[
    Product {
        id: "p-001",
        title: "Fresh Organic Tomatoes",
        vendor: "Kakata Farmers Co-op",
        price: "$4.50",
        image_url: "https://images.unsplash.com/photo-1592924357228-91a4daadcfea?w=500",
        category: "Groceries",
    },
    Product {
        id: "p-002",
        title: "Tecno Spark 10 Pro",
        vendor: "Monrovia Tech Hub",
        price: "$180",
        image_url: "https://images.unsplash.com/photo-1511707171634-5f897ff02aa9?w=500",
        category: "Electronics",
    },
    Product {
        id: "p-003",
        title: "Handwoven Liberian Shirt",
        vendor: "Heritage Apparel",
        price: "$25",
        image_url: "https://images.unsplash.com/photo-1596755094514-f87034a7a988?w=500",
        category: "Fashion",
    },
    Product {
        id: "p-004",
        title: "20kg Parboiled Rice",
        vendor: "City Provisions",
        price: "$18",
        image_url: "https://images.unsplash.com/photo-1586201375761-83865001e31c?w=500",
        category: "Groceries",
    },
    Product {
        id: "p-005",
        title: "Solar Power Bank 20000mAh",
        vendor: "ElectroWorld",
        price: "$30",
        image_url: "https://images.unsplash.com/photo-1619441207978-3d326c46e2c9?w=500",
        category: "Electronics",
    },
    Product {
        id: "p-006",
        title: "Locally Roasted Coffee",
        vendor: "Red Earth Roasters",
        price: "$8",
        image_url: "https://images.unsplash.com/photo-1559056199-641a0ac8b55e?w=500",
        category: "Agribusiness",
    },
    Product {
        id: "p-007",
        title: "Nike Air Max Sneakers",
        vendor: "StepUp Kicks",
        price: "$45",
        image_url: "https://images.unsplash.com/photo-1542291026-7eec264c27ff?w=500",
        category: "Fashion",
    },
    Product {
        id: "p-008",
        title: "Palm Oil (1 Gallon)",
        vendor: "Mama's Kitchen",
        price: "$12",
        image_url: "https://images.unsplash.com/photo-1620706122141-6f77ddec7afc?w=500",
        category: "Groceries",
    },
    Product {
        id: "p-009",
        title: "Bluetooth Speaker JBL",
        vendor: "SoundZone",
        price: "$55",
        image_url: "https://images.unsplash.com/photo-1608043152269-423dbba4e7e1?w=500",
        category: "Electronics",
    },
    Product {
        id: "p-010",
        title: "African Print Maxi Dress",
        vendor: "Bella Styles",
        price: "$22",
        image_url: "https://images.unsplash.com/photo-1490481651871-ab68de25d43d?w=500",
        category: "Fashion",
    },
    Product {
        id: "p-011",
        title: "100W Solar Panel",
        vendor: "GreenEnergy",
        price: "$85",
        image_url: "https://images.unsplash.com/photo-1509391366360-feaffa64d1c3?w=500",
        category: "Electronics",
    },
    Product {
        id: "p-012",
        title: "Casava Flour (5kg)",
        vendor: "Local Harvest",
        price: "$6",
        image_url: "https://images.unsplash.com/photo-1589113103503-4965ef2576b5?w=500",
        category: "Agribusiness",
    },
    Product {
        id: "p-013",
        title: "MacBook Pro M2 (Refurb)",
        vendor: "Monrovia Tech Hub",
        price: "$1,200",
        image_url: "https://images.unsplash.com/photo-1517336714731-489689fd1ca8?w=500",
        category: "Electronics",
    },
    Product {
        id: "p-014",
        title: "Shea Butter (500g)",
        vendor: "Natural Glow",
        price: "$10",
        image_url: "https://images.unsplash.com/photo-1608571423902-eed4a5ad8108?w=500",
        category: "Health & Beauty",
    },
    Product {
        id: "p-015",
        title: "Organic Honey 500ml",
        vendor: "Kakata Farmers Co-op",
        price: "$5",
        image_url: "https://images.unsplash.com/photo-1587049352846-4a222e784d38?w=500",
        category: "Groceries",
    },
];

#[component]
pub fn ProductCard(product: Product) -> Element {
    rsx! {
        Link {
            to: Route::Product { id: product.id.to_string() },
            class: "kh-product-card",
            img { class: "kh-product-image", src: "{product.image_url}", alt: "{product.title}" }
            div { class: "kh-product-details",
                p { class: "kh-product-title", "{product.title}" }
                p { class: "kh-product-vendor", "{product.vendor}" }
                div { class: "kh-product-footer",
                    p { class: "kh-product-price", "{product.price}" }
                    button {
                        class: "kh-add-btn",
                        aria_label: "Add to cart",
                        onclick: move |evt| {
                            evt.stop_propagation();
                        },
                        "+"
                    }
                }
            }
        }
    }
}

#[component]
pub fn Browse(query: String) -> Element {
    let mut search_query = use_signal(|| query.clone());
    let mut selected_category = use_signal(|| "All".to_string());
    let mut sort_by = use_signal(|| "Featured".to_string());
    let mut min_price = use_signal(|| 0.0);
    let mut max_price = use_signal(|| 2000.0);

    let categories = [
        "All",
        "Groceries",
        "Electronics",
        "Fashion",
        "Agribusiness",
        "Health & Beauty",
    ];

    let mut display_products: Vec<Product> = MOCK_PRODUCTS.iter().filter(|p| {
        let matches_search = search_query().is_empty()
            || p.title.to_lowercase().contains(&search_query().to_lowercase())
            || p.vendor.to_lowercase().contains(&search_query().to_lowercase());

        let matches_category = selected_category() == "All" || p.category == selected_category();
        let matches_price = p.price_val() >= min_price() && p.price_val() <= max_price();

        matches_search && matches_category && matches_price
    }).cloned().collect();

    // Sorting logic
    match sort_by().as_str() {
        "Price: Low to High" => display_products.sort_by(|a, b| a.price_val().partial_cmp(&b.price_val()).unwrap()),
        "Price: High to Low" => display_products.sort_by(|a, b| b.price_val().partial_cmp(&a.price_val()).unwrap()),
        _ => {}
    }

    rsx! {
        main { class: "kh-main-shell",
            div { class: "kh-animate-fade-in-up",
                div { style: "margin-bottom: 2.5rem;",
                    h1 { style: "font-size: 2.5rem; font-weight: var(--font-weight-bold); margin: 0 0 1rem;", "Find Everything" }
                    div { class: "kh-search-wrapper", style: "max-width: 100%;",
                        input {
                            class: "kh-search-input",
                            placeholder: "Search for electronics, fashion, groceries...",
                            value: "{search_query}",
                            oninput: move |evt| search_query.set(evt.value())
                        }
                    }
                }

                div { class: "kh-browse-layout",
                    // Sidebar Filters
                    aside { class: "kh-filter-sidebar",
                        div { class: "kh-filter-section",
                            h3 { class: "kh-filter-title", "Categories" }
                            div { style: "display: flex; flex-direction: column; gap: 0.5rem;",
                                for cat in categories {
                                    button {
                                        style: "text-align: left; background: none; border: none; padding: 0.5rem; cursor: pointer; border-radius: var(--radius-md); font-size: var(--text-sm);",
                                        style: if selected_category() == cat { "background: var(--color-brand-muted); color: var(--color-brand); font-weight: bold;" },
                                        onclick: move |_| selected_category.set(cat.to_string()),
                                        "{cat}"
                                    }
                                }
                            }
                        }

                        div { class: "kh-filter-section",
                            h3 { class: "kh-filter-title", "Price Range" }
                            div { class: "kh-price-inputs",
                                input { 
                                    class: "kh-price-input", 
                                    r#type: "number", 
                                    placeholder: "Min",
                                    oninput: move |evt| min_price.set(evt.value().parse().unwrap_or(0.0))
                                }
                                span { style: "color: var(--color-text-muted);", "—" }
                                input { 
                                    class: "kh-price-input", 
                                    r#type: "number", 
                                    placeholder: "Max",
                                    oninput: move |evt| max_price.set(evt.value().parse().unwrap_or(2000.0))
                                }
                            }
                        }

                        button { 
                            class: "kh-sidebar-link", 
                            style: "justify-content: center; margin-top: 1rem; border: 1px solid var(--color-border-subtle);",
                            onclick: move |_| {
                                selected_category.set("All".to_string());
                                min_price.set(0.0);
                                max_price.set(2000.0);
                            },
                            "Clear All" 
                        }
                    }

                    // Content Area
                    div {
                        div { class: "kh-sort-bar",
                            p { style: "font-weight: var(--font-weight-semibold); color: var(--color-text-secondary); margin: 0;", 
                                "Showing {display_products.len()} results" 
                            }
                            div { style: "display: flex; align-items: center; gap: 0.75rem;",
                                span { style: "font-size: var(--text-sm); color: var(--color-text-muted);", "Sort by:" }
                                select { 
                                    style: "padding: 0.4rem; border-radius: var(--radius-md); border: 1px solid var(--color-border-subtle); background: var(--color-surface); font-size: var(--text-sm);",
                                    onchange: move |evt| sort_by.set(evt.value()),
                                    option { "Featured" }
                                    option { "Price: Low to High" }
                                    option { "Price: High to Low" }
                                }
                            }
                        }

                        if display_products.is_empty() {
                            div { style: "text-align: center; padding: 6rem 2rem; background: var(--color-surface-elevated); border-radius: var(--radius-2xl); border: 2px dashed var(--color-border-subtle);",
                                span { style: "font-size: 3rem; display: block; margin-bottom: 1rem;", "🔍" }
                                h2 { "No results found" }
                                p { style: "color: var(--color-text-secondary);", "Try adjusting your filters or search query." }
                            }
                        } else {
                            div { class: "kh-product-grid",
                                for p in display_products {
                                    ProductCard { product: p.clone() }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
