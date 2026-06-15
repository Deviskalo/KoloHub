use crate::router::Route;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct TrackStage {
    label: &'static str,
    desc: &'static str,
    icon: &'static str,
    done: bool,
    active: bool,
}

#[component]
pub fn AccountTrack() -> Element {
    let stages = vec![
        TrackStage {
            label: "Order Placed",
            desc: "April 12, 2026 • 09:15 AM",
            icon: "✓",
            done: true,
            active: false,
        },
        TrackStage {
            label: "Confirmed",
            desc: "Vendor accepted your order",
            icon: "✓",
            done: true,
            active: false,
        },
        TrackStage {
            label: "Processing",
            desc: "Preparing your items",
            icon: "📦",
            done: false,
            active: true,
        },
        TrackStage {
            label: "On the Way",
            desc: "Courier is heading to you",
            icon: "🚚",
            done: false,
            active: false,
        },
        TrackStage {
            label: "Delivered",
            desc: "Package at your door",
            icon: "📍",
            done: false,
            active: false,
        },
    ];

    rsx! {
        main { class: "kh-main-shell",
            div { class: "kh-animate-fade-in-up",
                div { style: "display: flex; justify-content: space-between; align-items: baseline; margin-bottom: 2.5rem; border-bottom: 1px solid var(--color-border-subtle); padding-bottom: 1rem;",
                    h1 { style: "margin: 0; font-size: 2.5rem; font-weight: var(--font-weight-bold);", "Track Order" }
                    span { style: "color: var(--color-brand); font-weight: bold;", "Order #KH-259834" }
                }

                div { class: "kh-track-layout",
                    // Timeline Card
                    div { class: "kh-track-card",
                        div { class: "kh-track-header",
                            h2 { "Delivery Progress" }
                            p { "Estimated Delivery: Tomorrow, 2:00 PM - 4:00 PM" }
                        }

                        div { class: "kh-timeline",
                            for stage in stages {
                                div {
                                    class: "kh-timeline-step",
                                    class: if stage.done { "done" } else if stage.active { "active" } else { "" },

                                    div { class: "kh-timeline-icon", "{stage.icon}" }
                                    div { class: "kh-timeline-line" }
                                    div { class: "kh-timeline-content",
                                        p { class: "kh-timeline-title", "{stage.label}" }
                                        p { class: "kh-timeline-desc", "{stage.desc}" }
                                    }
                                }
                            }
                        }

                        div { style: "margin-top: 3rem;",
                            Link { to: Route::Home {},
                                crate::components::ui::Button { variant: "outline", "← Back to Marketplace" }
                            }
                        }
                    }

                    // Side Column: Courier & Map
                    div { style: "display: flex; flex-direction: column; gap: 2rem;",
                        // Courier Card
                        div { class: "kh-track-card",
                            div { class: "kh-track-header",
                                h2 { "Your Courier" }
                            }
                            div { style: "display: flex; align-items: center; gap: 1.5rem;",
                                img {
                                    class: "kh-courier-avatar",
                                    src: "https://images.unsplash.com/photo-1506794778202-cad84cf45f1d?auto=format&fit=crop&q=80&w=200",
                                    alt: "Courier Avatar"
                                }
                                div {
                                    p { style: "margin: 0; font-size: 1.15rem; font-weight: bold; color: var(--color-text-primary);", "Samson Kollie" }
                                    p { style: "margin: 0.25rem 0 0; font-size: var(--text-sm); color: var(--color-text-secondary);", "Verified Courier • 4.9 ★" }
                                    div { style: "margin-top: 0.75rem; display: flex; gap: 0.5rem;",
                                        button { style: "padding: 0.4rem 0.8rem; border-radius: var(--radius-md); border: 1px solid var(--color-brand); background: none; color: var(--color-brand); font-size: 0.75rem; font-weight: bold; cursor: pointer;",
                                            "Message"
                                        }
                                        button { style: "padding: 0.4rem 0.8rem; border-radius: var(--radius-md); background: var(--color-brand); border: none; color: #fff; font-size: 0.75rem; font-weight: bold; cursor: pointer;",
                                            "Call"
                                        }
                                    }
                                }
                            }
                        }

                        // Live Map Card
                        div { class: "kh-track-card",
                            div { class: "kh-track-header",
                                h2 { "Live Tracking" }
                            }
                            div {
                                style: "height: 300px; background: linear-gradient(135deg, var(--color-surface), var(--color-surface-elevated)); border: 1px solid var(--color-border-subtle); border-radius: var(--radius-xl); display: flex; flex-direction: column; align-items: center; justify-content: center; color: var(--color-text-muted); position: relative; overflow: hidden;",
                                // Simple visual for a map
                                div { style: "position: absolute; width: 100%; height: 100%; opacity: 0.1; background-image: radial-gradient(var(--color-brand) 1px, transparent 1px); background-size: 20px 20px;" }
                                span { style: "font-size: 3rem; margin-bottom: 1rem; z-index: 1;", "📍" }
                                p { style: "font-size: 0.85rem; font-weight: bold; z-index: 1;", "Connecting to GPS..." }
                                p { style: "font-size: 0.7rem; z-index: 1;", "Real-time tracking available soon" }
                            }
                        }
                    }
                }
            }
        }
    }
}
