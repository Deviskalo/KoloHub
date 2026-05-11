use dioxus::prelude::*;

#[component]
pub fn CustomerFlowStrip() -> Element {
    rsx! {
        section { class: "kh-customer-flow",
            div { style: "background: var(--color-surface-elevated); border: 1px solid var(--color-border-subtle); border-radius: var(--radius-xl); box-shadow: var(--shadow-card); padding: 0.9rem;",
                h2 { style: "margin: 0 0 0.75rem 0; color: var(--color-text-primary);",
                    "Customer Journey Flow"
                }
                div { class: "track",
                    div { class: "step",
                        p { class: "label", "1. Browse Products" }
                        p { class: "meta", "Categories, search, recommendations" }
                    }
                    div { class: "step",
                        p { class: "label", "2. Add to Cart" }
                        p { class: "meta", "Quantity updates and item review" }
                    }
                    div { class: "step",
                        p { class: "label", "3. Checkout & Pay" }
                        p { class: "meta", "Mobile Money, COD, or card" }
                    }
                    div { class: "step",
                        p { class: "label", "4. Track Delivery" }
                        p { class: "meta", "Status stages + tracking ID" }
                    }
                    div { class: "step",
                        p { class: "label", "5. Rate & Review" }
                        p { class: "meta", "Post-delivery trust loop" }
                    }
                }
            }
        }
    }
}
