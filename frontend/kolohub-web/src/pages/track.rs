use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct TrackStage {
    label: &'static str,
    desc: &'static str,
    icon: &'static str,
    done: bool,
    active: bool,
}

/// Delivery tracking page — Phase 8.5
#[component]
pub fn Track() -> Element {
    let stages = vec![
        TrackStage {
            label: "Order Placed",
            desc: "We received your order.",
            icon: "✓",
            done: true,
            active: false,
        },
        TrackStage {
            label: "Confirmed",
            desc: "Vendor has processed the request.",
            icon: "✓",
            done: true,
            active: false,
        },
        TrackStage {
            label: "Packaged",
            desc: "Ready for pickup.",
            icon: "📦",
            done: false,
            active: true,
        },
        TrackStage {
            label: "In Transit",
            desc: "Courier is on the way.",
            icon: "🚚",
            done: false,
            active: false,
        },
        TrackStage {
            label: "Delivered",
            desc: "Package handed over.",
            icon: "📍",
            done: false,
            active: false,
        },
    ];

    rsx! {
        main { class: "kh-main-shell",
            div { style: "padding-top: 2rem;",
                h1 {
                    style: "margin: 0 0 1.5rem; font-size: var(--text-2xl); font-weight: var(--font-weight-bold); color: var(--color-text-primary);",
                    "Track Delivery"
                }

                div { class: "kh-track-layout",
                    // Timeline Card
                    div { class: "kh-track-card",
                        div { class: "kh-track-header",
                            h2 { "Order Status" }
                            p { "Order #KH-259834 • Arriving Tomorrow" }
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

                        a {
                            href: "/",
                            style: "display: flex; align-items: center; justify-content: center; height: 3rem; margin-top: 2rem; border-radius: var(--radius-md); border: 1px solid var(--color-border-subtle); color: var(--color-text-primary); font-size: var(--text-sm); font-weight: var(--font-weight-semibold); text-decoration: none;",
                            "← Back to Home"
                        }
                    }

                    // Courier/Map Placeholder Card
                    div { class: "kh-track-card",
                        div { class: "kh-track-header",
                            h2 { "Courier Details" }
                        }
                        div { style: "display: flex; align-items: center; gap: 1rem; margin-top: 1.5rem;",
                            div { style: "width: 4rem; height: 4rem; border-radius: 50%; background: var(--color-border-subtle);" }
                            div {
                                p { style: "margin: 0; font-weight: var(--font-weight-semibold); color: var(--color-text-primary);", "Samson Kollie" }
                                p { style: "margin: 0.25rem 0 0; font-size: var(--text-sm); color: var(--color-text-secondary);", "Monrovia Logistics" }
                            }
                        }
                        div {
                            style: "margin-top: 2rem; height: 16rem; background: var(--color-surface); border: 1px solid var(--color-border-subtle); border-radius: var(--radius-lg); display: flex; align-items: center; justify-content: center; color: var(--color-text-muted);",
                            "[ Map View Placeholder ]"
                        }
                    }
                }
            }
        }
    }
}
