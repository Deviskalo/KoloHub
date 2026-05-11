use dioxus::prelude::*;

/// Vendor dashboard page — Phase 9.
#[component]
pub fn Vendor() -> Element {
    rsx! {
        main {
            style: "max-width: 72rem; margin: 0 auto; padding: 8rem 1rem 3rem;",
            div {
                style: "background: var(--color-surface-elevated); border: 1px solid var(--color-border-subtle); border-radius: var(--radius-xl); box-shadow: var(--shadow-card); padding: 2rem; text-align: center;",
                span { style: "font-size: 2.5rem;", "🏪" }
                h1 {
                    style: "margin: 0.75rem 0 0.5rem; font-size: var(--text-2xl); font-weight: var(--font-weight-bold); color: var(--color-text-primary);",
                    "Vendor & Admin Panel"
                }
                p {
                    style: "margin: 0 auto; max-width: 36rem; color: var(--color-text-secondary);",
                    "The full Vendor dashboard — product management, order fulfillment, and sales analytics — is coming in Phase 9."
                }
                div { style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 0.75rem; max-width: 28rem; margin: 1.5rem auto 0;",
                    div {
                        style: "border: 1px solid var(--color-border-subtle); border-radius: var(--radius-md); padding: 0.75rem;",
                        p { style: "margin: 0; font-size: var(--text-xs); color: var(--color-text-muted);", "Products" }
                        p { style: "margin: 0.15rem 0 0; font-size: var(--text-base); font-weight: var(--font-weight-semibold); color: var(--color-text-primary);", "—" }
                    }
                    div {
                        style: "border: 1px solid var(--color-border-subtle); border-radius: var(--radius-md); padding: 0.75rem;",
                        p { style: "margin: 0; font-size: var(--text-xs); color: var(--color-text-muted);", "Orders" }
                        p { style: "margin: 0.15rem 0 0; font-size: var(--text-base); font-weight: var(--font-weight-semibold); color: var(--color-text-primary);", "—" }
                    }
                    div {
                        style: "border: 1px solid var(--color-border-subtle); border-radius: var(--radius-md); padding: 0.75rem;",
                        p { style: "margin: 0; font-size: var(--text-xs); color: var(--color-text-muted);", "Revenue" }
                        p { style: "margin: 0.15rem 0 0; font-size: var(--text-base); font-weight: var(--font-weight-semibold); color: var(--color-text-primary);", "—" }
                    }
                }
                a {
                    href: "/",
                    style: "display: inline-flex; align-items: center; margin-top: 1.5rem; height: 2.5rem; padding: 0 1.25rem; border-radius: var(--radius-md); background: var(--color-brand); color: #fff; font-size: var(--text-sm); font-weight: var(--font-weight-semibold); text-decoration: none;",
                    "← Back to Home"
                }
            }
        }
    }
}
