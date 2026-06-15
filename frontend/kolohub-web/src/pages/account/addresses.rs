use dioxus::prelude::*;
use crate::components::ui::Button;

#[derive(Clone, PartialEq)]
struct Address {
    label: String,
    content: String,
    is_default: bool,
}

#[component]
pub fn AccountAddresses() -> Element {
    let addresses = vec![
        Address {
            label: "Home".to_string(),
            content: "House 24, Airfield Shortcut, Sinkor, Monrovia".to_string(),
            is_default: true,
        },
        Address {
            label: "Office".to_string(),
            content: "12th Street, Beach Side, Sinkor, Monrovia".to_string(),
            is_default: false,
        },
    ];

    rsx! {
        div { class: "kh-animate-fade-in-up",
            div { 
                style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 2rem;",
                h2 { style: "font-size: 1.5rem; font-weight: var(--font-weight-bold);", "Saved Addresses" }
                Button { variant: "brand", "+ Add New Address" }
            }

            div { 
                style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 1.5rem;",
                for addr in addresses {
                    div { class: "kh-dash-card", key: "{addr.label}",
                        div { style: "padding: 1.5rem;",
                            div { 
                                style: "display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 1rem;",
                                div {
                                    span { 
                                        style: "font-size: var(--text-xs); font-weight: var(--font-weight-bold); text-transform: uppercase; color: var(--color-brand);", 
                                        "{addr.label}" 
                                    }
                                    if addr.is_default {
                                        span { 
                                            style: "margin-left: 0.5rem; font-size: 0.6rem; padding: 0.1rem 0.4rem; background: var(--color-brand-muted); color: var(--color-brand); border-radius: var(--radius-sm);",
                                            "DEFAULT"
                                        }
                                    }
                                }
                                div { style: "display: flex; gap: 0.5rem;",
                                    button { style: "background: none; border: none; cursor: pointer; font-size: 1rem;", "✏️" }
                                    button { style: "background: none; border: none; cursor: pointer; font-size: 1rem; color: #ef4444;", "🗑️" }
                                }
                            }
                            p { 
                                style: "font-size: var(--text-sm); color: var(--color-text-secondary); line-height: 1.6;", 
                                "{addr.content}" 
                            }
                            
                            if !addr.is_default {
                                button { 
                                    style: "margin-top: 1.5rem; background: none; border: none; cursor: pointer; font-size: var(--text-xs); color: var(--color-brand); font-weight: var(--font-weight-bold); padding: 0;",
                                    "Set as Default" 
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
