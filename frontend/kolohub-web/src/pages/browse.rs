use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct Product {
    id: &'static str,
    title: &'static str,
    vendor: &'static str,
    price: &'static str,
    image_url: &'static str,
}

const MOCK_PRODUCTS: &[Product] = &[
    Product {
        id: "p-001",
        title: "Fresh Organic Tomatoes",
        vendor: "Kakata Farmers Co-op",
        price: "$4.50",
        image_url: "/images/design-ui.png", // Using the placeholder we have
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
];

#[component]
fn ProductCard(product: Product) -> Element {
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
                            // Cart logic to be implemented
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
pub fn Browse() -> Element {
    let mut selected_category = use_signal(|| "All".to_string());
    let categories = [
        "All",
        "Groceries",
        "Electronics",
        "Fashion",
        "Home & Garden",
    ];

    rsx! {
        main { class: "kh-main-shell",
            div { style: "display: flex; flex-direction: column; gap: 0.5rem;",
                h1 {
                    style: "margin: 0; font-size: var(--text-2xl); font-weight: var(--font-weight-bold); color: var(--color-text-primary);",
                    "Browse Products"
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

                // Product Grid
                div { class: "kh-product-grid",
                    for p in MOCK_PRODUCTS {
                        ProductCard { product: p.clone() }
                    }
                }
            }
        }
    }
}
