use crate::router::Route;
use dioxus::prelude::*;

#[component]
pub fn AccountOrders() -> Element {
    let mock_orders = [
        ("#KH-4501", "May 12, 2026", "LRD 4,500", "In Transit", "https://images.unsplash.com/photo-1592924357228-91a4daadcfea?w=200"),
        ("#KH-4498", "May 08, 2026", "LRD 1,200", "Delivered", "https://images.unsplash.com/photo-1559056199-641a0ac8b55e?w=200"),
        ("#KH-4492", "Apr 28, 2026", "LRD 180", "Delivered", "https://images.unsplash.com/photo-1619441207978-3d326c46e2c9?w=200"),
    ];

    rsx! {
        div { class: "kh-animate-fade-in-up",
            div { style: "display: flex; flex-direction: column; gap: 1.5rem;",
                for (id, date, total, status, img) in mock_orders {
                    div { class: "kh-order-history-item",
                        div { style: "display: flex; gap: 1.5rem; align-items: center;",
                            img { 
                                src: "{img}", 
                                style: "width: 5rem; height: 5rem; border-radius: var(--radius-lg); object-fit: cover; border: 1px solid var(--color-border-subtle);" 
                            }
                            div {
                                p { style: "margin: 0; font-size: var(--text-xs); color: var(--color-text-muted); text-transform: uppercase; letter-spacing: 0.05em;", "Order {id}" }
                                h4 { style: "margin: 0.25rem 0; font-size: 1.1rem; font-weight: var(--font-weight-bold);", "Placed on {date}" }
                                p { style: "margin: 0; font-weight: var(--font-weight-semibold); color: var(--color-brand);", "{total}" }
                            }
                        }
                        
                        div { style: "display: flex; flex-direction: column; align-items: flex-end; gap: 0.75rem;",
                            span { 
                                class: "kh-badge-status",
                                class: if status == "In Transit" { "warning" } else { "success" },
                                "{status}" 
                            }
                            div { style: "display: flex; gap: 0.5rem;",
                                if status == "In Transit" {
                                    Link { to: Route::Track {},
                                        crate::components::ui::Button { variant: "brand", "Track Order" }
                                    }
                                }
                                Link { to: Route::AccountOrderDetails { id: id.replace("#", "") },
                                    crate::components::ui::Button { variant: "outline", "View Details" }
                                }
                            }
                        }
                    }
                }
            }

            if mock_orders.is_empty() {
                div { style: "text-align: center; padding: 4rem 2rem; background: var(--color-surface-elevated); border-radius: var(--radius-2xl); border: 2px dashed var(--color-border-subtle);",
                    span { style: "font-size: 3rem; display: block; margin-bottom: 1rem;", "🛍️" }
                    h2 { "No orders yet" }
                    p { style: "color: var(--color-text-secondary); margin-bottom: 2rem;", "Explore the marketplace to find fresh local products." }
                    Link { to: Route::Browse { query: "".to_string() },
                        crate::components::ui::Button { variant: "brand", "Start Shopping" }
                    }
                }
            }
        }
    }
}
