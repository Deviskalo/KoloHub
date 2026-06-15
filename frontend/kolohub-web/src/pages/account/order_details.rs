use crate::router::Route;
use dioxus::prelude::*;

#[component]
pub fn AccountOrderDetails(id: String) -> Element {
    rsx! {
        div { class: "kh-animate-fade-in-up",
            div { class: "kh-dashboard-header",
                style: "display: flex; justify-content: space-between; align-items: center;",
                div {
                    h1 { class: "kh-dashboard-title", "Order Details" }
                    p { style: "color: var(--color-text-secondary); margin: 0.25rem 0 0;", "Order {id} — Placed on May 12, 2026" }
                }
                Link { to: Route::AccountOrders {},
                    crate::components::ui::Button { variant: "outline", "Back to Orders" }
                }
            }

            div { class: "kh-dash-row",
                // Order Items & Summary
                div { style: "display: flex; flex-direction: column; gap: 2rem;",
                    div { class: "kh-dash-card",
                        div { class: "kh-dash-card-header", "Items in this Order" }
                        div { class: "kh-activity-list",
                            for (name, qty, price, img) in [
                                ("Organic Liberian Coffee", "2", "LRD 3,000", "https://images.unsplash.com/photo-1559056199-641a0ac8b55e?w=200"),
                                ("Fresh Palm Butter (1L)", "1", "LRD 1,500", "https://images.unsplash.com/photo-1592924357228-91a4daadcfea?w=200"),
                            ] {
                                div { class: "kh-activity-item",
                                    div { style: "display: flex; gap: 1rem; align-items: center;",
                                        img { src: "{img}", style: "width: 4rem; height: 4rem; border-radius: var(--radius-lg); object-fit: cover;" }
                                        div {
                                            p { class: "kh-activity-title", "{name}" }
                                            p { class: "kh-activity-sub", "Quantity: {qty}" }
                                        }
                                    }
                                    p { style: "font-weight: var(--font-weight-bold);", "{price}" }
                                }
                            }
                        }
                        
                        div { style: "padding: 2rem; background: var(--color-surface); border-top: 1px solid var(--color-border-subtle);",
                            div { style: "display: flex; flex-direction: column; gap: 0.75rem;",
                                div { style: "display: flex; justify-content: space-between; font-size: var(--text-sm);",
                                    span { "Subtotal" }
                                    span { "LRD 4,500" }
                                }
                                div { style: "display: flex; justify-content: space-between; font-size: var(--text-sm);",
                                    span { "Shipping Fee" }
                                    span { "LRD 250" }
                                }
                                div { style: "display: flex; justify-content: space-between; font-size: 1.1rem; font-weight: var(--font-weight-bold); margin-top: 0.5rem; color: var(--color-brand);",
                                    span { "Total" }
                                    span { "LRD 4,750" }
                                }
                            }
                        }
                    }
                }

                // Shipping & Payment Column
                div { style: "display: flex; flex-direction: column; gap: 2rem;",
                    div { class: "kh-dash-card",
                        div { class: "kh-dash-card-header", "Delivery Address" }
                        div { style: "padding: 1.5rem;",
                            p { style: "font-weight: var(--font-weight-bold); margin-bottom: 0.5rem;", "Johnathan Doe" }
                            p { style: "font-size: var(--text-sm); color: var(--color-text-secondary); line-height: 1.5;", 
                                "House 24, Airfield Shortcut"
                                br {}
                                "Sinkor, Monrovia"
                                br {}
                                "Liberia"
                            }
                            p { style: "font-size: var(--text-sm); color: var(--color-text-secondary); margin-top: 1rem;", "+231 88 000 0000" }
                        }
                    }

                    div { class: "kh-dash-card",
                        div { class: "kh-dash-card-header", "Payment Method" }
                        div { style: "padding: 1.5rem; display: flex; align-items: center; gap: 1rem;",
                            span { style: "font-size: 1.5rem;", "📱" }
                            div {
                                p { style: "font-weight: var(--font-weight-bold); margin: 0;", "Orange Money" }
                                p { style: "font-size: var(--text-xs); color: var(--color-text-muted);", "Transaction: #OM-99283" }
                            }
                        }
                    }

                    div { class: "kh-dash-card",
                        div { class: "kh-dash-card-header", "Actions" }
                        div { style: "padding: 1.5rem; display: flex; flex-direction: column; gap: 1rem;",
                            crate::components::ui::Button { variant: "brand", style_attr: "width: 100%;", "Download Invoice" }
                            crate::components::ui::Button { variant: "outline", style_attr: "width: 100%;", "Reorder Items" }
                            Link { to: Route::Track {},
                                crate::components::ui::Button { variant: "accent", style_attr: "width: 100%;", "Track Shipment" }
                            }
                        }
                    }
                }
            }
        }
    }
}
