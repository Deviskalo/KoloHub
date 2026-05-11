use dioxus::prelude::*;

/// Checkout & payment page — Phase 8.
#[component]
pub fn Checkout() -> Element {
    rsx! {
        main {
            style: "max-width: 72rem; margin: 0 auto; padding: 8rem 1rem 3rem;",
            div {
                style: "background: var(--color-surface-elevated); border: 1px solid var(--color-border-subtle); border-radius: var(--radius-xl); box-shadow: var(--shadow-card); padding: 2rem; text-align: center;",
                span { style: "font-size: 2.5rem;", "💳" }
                h1 {
                    style: "margin: 0.75rem 0 0.5rem; font-size: var(--text-2xl); font-weight: var(--font-weight-bold); color: var(--color-text-primary);",
                    "Checkout & Pay"
                }
                p {
                    style: "margin: 0 auto; max-width: 36rem; color: var(--color-text-secondary);",
                    "Secure checkout with Mobile Money and Cash on Delivery is coming in Phase 8."
                }
                div { style: "display: flex; gap: 0.6rem; justify-content: center; margin-top: 1.5rem; flex-wrap: wrap;",
                    span {
                        style: "display: inline-flex; align-items: center; gap: 0.4rem; padding: 0.35rem 0.75rem; border-radius: var(--radius-md); background: var(--color-brand-muted); color: var(--color-brand-foreground); font-size: var(--text-xs); font-weight: var(--font-weight-semibold);",
                        "📱 Mobile Money"
                    }
                    span {
                        style: "display: inline-flex; align-items: center; gap: 0.4rem; padding: 0.35rem 0.75rem; border-radius: var(--radius-md); background: var(--color-brand-muted); color: var(--color-brand-foreground); font-size: var(--text-xs); font-weight: var(--font-weight-semibold);",
                        "💵 Cash on Delivery"
                    }
                }
                a {
                    href: "/cart",
                    style: "display: inline-flex; align-items: center; margin-top: 1.5rem; height: 2.5rem; padding: 0 1.25rem; border-radius: var(--radius-md); border: 1px solid var(--color-border-subtle); color: var(--color-text-primary); font-size: var(--text-sm); font-weight: var(--font-weight-semibold); text-decoration: none;",
                    "← Back to Cart"
                }
            }
        }
    }
}
