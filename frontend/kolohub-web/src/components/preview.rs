use dioxus::prelude::*;

#[component]
pub fn MarketplacePreview() -> Element {
    rsx! {
        main { class: "kh-main-shell",
            div { class: "kh-preview-grid",
                section {
                    class: "kh-hero-banner",
                    style: "border-radius: var(--radius-xl); padding: 1.5rem; display: flex; flex-direction: column; justify-content: center;",
                    h1 { style: "font-size: var(--text-4xl); font-weight: var(--font-weight-bold); margin: 0 0 0.5rem 0;",
                        "Discover. Shop. Grow."
                    }
                    p { style: "margin: 0 0 0.35rem 0;",
                        "KoloHub connects buyers and vendors in a trusted local marketplace — mobile-first, Mobile Money ready."
                    }
                    p { style: "margin: 0 0 1rem 0; font-size: var(--text-sm);",
                        "Browse products, track deliveries, manage your store — all in one platform."
                    }
                    div { style: "display: flex; gap: 0.75rem; margin-top: 0.5rem; flex-wrap: wrap;",
                        a { href: "/browse", class: "kh-btn-accent", "Browse Products" }
                        a { href: "/vendor", class: "kh-btn-brand", "Vendor & Admin Panel" }
                    }
                    div { style: "margin-top: auto; padding-top: 1rem; display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 0.5rem;",
                        div { style: "border: 1px solid rgba(255,255,255,0.3); background: rgba(0,0,0,0.4); border-radius: var(--radius-md); padding: 0.5rem; backdrop-filter: blur(4px);",
                            p { style: "margin: 0; color: rgba(255,255,255,0.8); font-size: var(--text-xs);",
                                "Currency"
                            }
                            p { style: "margin: 0.2rem 0 0; color: #ffffff; font-weight: var(--font-weight-semibold);",
                                "LRD / USD"
                            }
                        }
                        div { style: "border: 1px solid rgba(255,255,255,0.3); background: rgba(0,0,0,0.4); border-radius: var(--radius-md); padding: 0.5rem; backdrop-filter: blur(4px);",
                            p { style: "margin: 0; color: rgba(255,255,255,0.8); font-size: var(--text-xs);",
                                "Flow Stages"
                            }
                            p { style: "margin: 0.2rem 0 0; color: #ffffff; font-weight: var(--font-weight-semibold);",
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
