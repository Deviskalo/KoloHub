use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Product {
    pub id: &'static str,
    pub title: &'static str,
    pub vendor: &'static str,
    pub price: &'static str,
    pub image_url: &'static str,
}

pub const MOCK_PRODUCTS: &[Product] = &[
    Product {
        id: "p-001",
        title: "Fresh Organic Tomatoes",
        vendor: "Kakata Farmers Co-op",
        price: "$4.50",
        image_url: "/images/design-ui.png",
    },
    Product {
        id: "p-002",
        title: "Tecno Spark 10 Pro",
        vendor: "Monrovia Tech Hub",
        price: "$180",
        image_url: "/images/design-ui.png",
    },
    Product {
        id: "p-003",
        title: "Handwoven Liberian Shirt",
        vendor: "Heritage Apparel",
        price: "$25",
        image_url: "/images/design-ui.png",
    },
    Product {
        id: "p-004",
        title: "20kg Parboiled Rice",
        vendor: "City Provisions",
        price: "$18",
        image_url: "/images/design-ui.png",
    },
    Product {
        id: "p-005",
        title: "Solar Power Bank 20000mAh",
        vendor: "ElectroWorld",
        price: "$30",
        image_url: "/images/design-ui.png",
    },
    Product {
        id: "p-006",
        title: "Locally Roasted Coffee",
        vendor: "Red Earth Roasters",
        price: "$8",
        image_url: "/images/design-ui.png",
    },
    Product {
        id: "p-007",
        title: "Nike Air Max Sneakers",
        vendor: "StepUp Kicks",
        price: "$45",
        image_url: "/images/design-ui.png",
    },
    Product {
        id: "p-008",
        title: "Palm Oil (1 Gallon)",
        vendor: "Mama's Kitchen",
        price: "$12",
        image_url: "/images/design-ui.png",
    },
    Product {
        id: "p-009",
        title: "Bluetooth Speaker JBL",
        vendor: "SoundZone",
        price: "$55",
        image_url: "/images/design-ui.png",
    },
    Product {
        id: "p-010",
        title: "African Print Maxi Dress",
        vendor: "Bella Styles",
        price: "$22",
        image_url: "/images/design-ui.png",
    },
    Product {
        id: "p-011",
        title: "100W Solar Panel",
        vendor: "GreenEnergy",
        price: "$85",
        image_url: "/images/design-ui.png",
    },
    Product {
        id: "p-012",
        title: "Casava Flour (5kg)",
        vendor: "Local Harvest",
        price: "$6",
        image_url: "/images/design-ui.png",
    },
    Product {
        id: "p-013",
        title: "MacBook Pro M2 (Refurb)",
        vendor: "Monrovia Tech Hub",
        price: "$950",
        image_url: "/images/design-ui.png",
    },
    Product {
        id: "p-014",
        title: "Handmade Leather Sandals",
        vendor: "Heritage Apparel",
        price: "$15",
        image_url: "/images/design-ui.png",
    },
    Product {
        id: "p-015",
        title: "Organic Honey 500ml",
        vendor: "Kakata Farmers Co-op",
        price: "$5",
        image_url: "/images/design-ui.png",
    },
];

#[component]
pub fn ProductCard(product: Product) -> Element {
    rsx! {
        a {
            class: "kh-product-card",
            href: "/product/{product.id}",
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

/// Product browse page — Phase 8.1
#[component]
pub fn Browse(query: String) -> Element {
    let mut selected_category = use_signal(|| "All".to_string());

    let mut search_query = use_signal(|| query.clone());

    let categories = [
        "All",
        "Groceries",
        "Electronics",
        "Fashion",
        "Home & Garden",
    ];

    let display_products = MOCK_PRODUCTS.iter().filter(|p| {
        let matches_search = search_query().is_empty()
            || p.title
                .to_lowercase()
                .contains(&search_query().to_lowercase())
            || p.vendor
                .to_lowercase()
                .contains(&search_query().to_lowercase());

        matches_search
    });

    rsx! {
        main { class: "kh-main-shell",
            div { style: "display: flex; flex-direction: column; gap: 0.5rem;",

                div { style: "display: flex; flex-direction: column; gap: 1rem; margin-bottom: 0.5rem;",
                    h1 {
                        style: "margin: 0; font-size: var(--text-2xl); font-weight: var(--font-weight-bold); color: var(--color-text-primary);",
                        "Browse Products"
                    }
                    div { class: "kh-search-wrapper", style: "margin: 0; max-width: 100%;",
                        input {
                            class: "kh-search-input",
                            placeholder: "Search catalog...",
                            value: "{search_query}",
                            oninput: move |evt| search_query.set(evt.value())
                        }
                    }
                }

                // Category Strip
                div { class: "kh-category-strip",
                    for cat in categories {
                        button {
                            class: "kh-category-chip",
                            class: if selected_category() == cat { "active" },
                            onclick: move |_| selected_category.set(cat.to_string()),
                            "{cat}"
                        }
                    }
                }

                p { style: "color: var(--color-text-secondary);", "Showing results: {display_products.clone().count()}"}

                // Product Grid
                div { class: "kh-product-grid",
                    for p in display_products {
                        ProductCard { product: p.clone() }
                    }
                }
            }
        }
    }
}
