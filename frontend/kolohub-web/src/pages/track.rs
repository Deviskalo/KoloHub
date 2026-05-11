use dioxus::prelude::*;

/// Delivery tracking page — Phase 8.
#[component]
pub fn Track() -> Element {
    rsx! {
        main {
            style: "max-width: 72rem; margin: 0 auto; padding: 8rem 1rem 3rem;",
            div {
                style: "background: var(--color-surface-elevated); border: 1px solid var(--color-border-subtle); border-radius: var(--radius-xl); box-shadow: var(--shadow-card); padding: 2rem; text-align: center;",
                span { style: "font-size: 2.5rem;", "📦" }
                h1 {
                    style: "margin: 0.75rem 0 0.5rem; font-size: var(--text-2xl); font-weight: var(--font-weight-bold); color: var(--color-text-primary);",
                    "Track Your Delivery"
                }
                p {
                    style: "margin: 0 auto; max-width: 36rem; color: var(--color-text-secondary);",
                    "Real-time order status and courier tracking is coming in Phase 8."
                }
                div { style: "display: grid; grid-template-columns: repeat(5, 1fr); gap: 0.5rem; max-width: 36rem; margin: 1.5rem auto 0;",
                    for (label, done) in [("Placed", true), ("Confirmed", true), ("Packaged", false), ("In Transit", false), ("Delivered", false)] {
                        div {
                            style: if done {
                                "border: 1px solid var(--color-brand); border-radius: var(--radius-md); padding: 0.5rem 0.25rem; background: var(--color-brand-muted);"
                            } else {
                                "border: 1px solid var(--color-border-subtle); border-radius: var(--radius-md); padding: 0.5rem 0.25rem; background: var(--color-surface);"
                            },
                            p {
                                style: "margin: 0; font-size: var(--text-xs); font-weight: var(--font-weight-semibold); color: var(--color-text-primary);",
                                "{label}"
                            }
                        }
                    }
                }
                a {
                    href: "/",
                    style: "display: inline-flex; align-items: center; margin-top: 1.5rem; height: 2.5rem; padding: 0 1.25rem; border-radius: var(--radius-md); border: 1px solid var(--color-border-subtle); color: var(--color-text-primary); font-size: var(--text-sm); font-weight: var(--font-weight-semibold); text-decoration: none;",
                    "← Back to Home"
                }
            }
        }
    }
}
