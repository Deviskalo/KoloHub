use dioxus::prelude::*;

#[component]
pub fn MarketplacePreview(brand: &'static str) -> Element {
    rsx! {
        main { class: "kh-main-shell",
            div { class: "kh-preview-grid",
                section {
                    class: "kh-surface-glass",
                    style: "border-radius: var(--radius-xl); padding: 1.25rem; display: flex; flex-direction: column;",
                    h1 { style: "font-size: var(--text-4xl); font-weight: var(--font-weight-bold); margin: 0 0 0.5rem 0; color: var(--color-text-primary);",
                        "Frontend Preview"
                    }
                    p { style: "margin: 0 0 1rem 0; color: var(--color-text-secondary);",
                        "Modern, Fast + Mobile First"
                    }
                    p { style: "margin: 0; color: var(--color-text-secondary);",
                        "Built from Design-UI with design-system tokens. Brand purple: {brand}"
                    }
                    div { style: "display: flex; gap: 0.75rem; margin-top: 1rem; flex-wrap: wrap;",
                        a { href: "#", class: "kh-btn-accent", "Customer App" }
                        a { href: "#", class: "kh-btn-brand", "Vendor & Admin Panel" }
                    }
                    div { style: "margin-top: auto; padding-top: 1rem; display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 0.5rem;",
                        div { style: "border: 1px solid var(--color-border-subtle); border-radius: var(--radius-md); padding: 0.5rem;",
                            p { style: "margin: 0; color: var(--color-text-muted); font-size: var(--text-xs);",
                                "Currency"
                            }
                            p { style: "margin: 0.2rem 0 0; color: var(--color-text-primary); font-weight: var(--font-weight-semibold);",
                                "UGX"
                            }
                        }
                        div { style: "border: 1px solid var(--color-border-subtle); border-radius: var(--radius-md); padding: 0.5rem;",
                            p { style: "margin: 0; color: var(--color-text-muted); font-size: var(--text-xs);",
                                "Flow Stages"
                            }
                            p { style: "margin: 0.2rem 0 0; color: var(--color-text-primary); font-weight: var(--font-weight-semibold);",
                                "Browse → Track"
                            }
                        }
                    }
                }

                img {
                    src: "/images/design-ui.png",
                    alt: "KoloHub Design UI preview",
                    class: "kh-preview-image",
                }
            }
        }
    }
}
