use dioxus::prelude::*;

#[component]
pub fn Product(id: String) -> Element {
    // In a real app we would load the product details by `id` from backend.
    // Here we use a generic mock state for the prototype.
    rsx! {
        main { class: "kh-main-shell",
            div { class: "kh-customer-flow", style: "padding-top: 2rem;",
                a {
                    href: "/browse",
                    style: "display: inline-flex; align-items: center; margin-bottom: 1.5rem; text-decoration: none; color: var(--color-text-secondary); font-size: var(--text-sm); font-weight: var(--font-weight-medium);",
                    "← Back to Products"
                }

                div { class: "kh-pdp-layout",
                    // Image Column
                    div { class: "kh-pdp-image-container",
                        img {
                            src: "/images/design-ui.png",
                            alt: "Product Image",
                            class: "kh-pdp-image"
                        }
                    }

                    // Content Column
                    div { class: "kh-pdp-content",
                        div { class: "kh-pdp-header",
                            span { class: "kh-pdp-badge", "In Stock" }
                            h1 { class: "kh-pdp-title", "Mock Product Title ({id})" }
                            p { class: "kh-pdp-vendor", "By Awesome Vendor Co." }
                        }

                        p { class: "kh-pdp-price", "$45.00" }

                        div {
                            h3 { style: "font-size: var(--text-lg); font-weight: var(--font-weight-bold); color: var(--color-text-primary); margin: 1rem 0 0.5rem;", "Details" }
                            p { class: "kh-pdp-desc",
                                "This is a detailed description of the product. It includes material information, sizing, and any other relevant data a customer might need before purchasing."
                            }
                        }

                        // CTA Wrap (sticky on mobile, inline on desktop)
                        div { class: "kh-pdp-cta-wrap",
                            button {
                                class: "kh-btn-brand kh-btn-lg",
                                style: "font-weight: var(--font-weight-bold);",
                                "Add to Cart — $45.00"
                            }
                        }
                    }
                }
            }
        }
    }
}
