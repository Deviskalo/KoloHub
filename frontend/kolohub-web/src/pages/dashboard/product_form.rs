use dioxus::prelude::*;

#[component]
pub fn DashboardProductAdd() -> Element {
    rsx! {
        DashboardProductForm { id: None }
    }
}

#[component]
pub fn DashboardProductEdit(id: String) -> Element {
    rsx! {
        DashboardProductForm { id: Some(id) }
    }
}

#[component]
fn DashboardProductForm(id: Option<String>) -> Element {
    let is_edit = id.is_some();
    let title = if is_edit { "Edit Product" } else { "Add New Product" };
    let sub_text = if is_edit { "update your" } else { "list a new" };
    let button_text = if is_edit { "Save Changes" } else { "List Product" };

    rsx! {
        div { class: "kh-animate-fade-in-up",
            div { class: "kh-dashboard-header",
                style: "display: flex; justify-content: space-between; align-items: center;",
                div {
                    h1 { class: "kh-dashboard-title", "{title}" }
                    p { style: "color: var(--color-text-secondary); margin: 0.25rem 0 0;", "Fill in the details below to {sub_text} product." }
                }
                Link { to: crate::router::Route::DashboardProducts {},
                    crate::components::ui::Button { variant: "outline", "Cancel" }
                }
            }

            div { class: "kh-dash-row",
                // Main Form Card
                div { class: "kh-dash-card",
                    div { class: "kh-dash-card-header", "Basic Information" }
                    div { style: "padding: 2rem; display: flex; flex-direction: column; gap: 1.5rem;",
                        div {
                            label { style: "display: block; font-size: var(--text-sm); font-weight: bold; margin-bottom: 0.5rem;", "Product Name" }
                            input { class: "kh-input", placeholder: "e.g. Organic Liberian Coffee" }
                        }
                        div {
                            label { style: "display: block; font-size: var(--text-sm); font-weight: bold; margin-bottom: 0.5rem;", "Description" }
                            textarea { 
                                class: "kh-input", 
                                style: "min-height: 8rem; resize: vertical; padding: 1rem;",
                                placeholder: "Describe your product in detail..." 
                            }
                        }
                        div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 1.5rem;",
                            div {
                                label { style: "display: block; font-size: var(--text-sm); font-weight: bold; margin-bottom: 0.5rem;", "Category" }
                                select { class: "kh-input",
                                    option { "Select Category" }
                                    option { "Groceries" }
                                    option { "Electronics" }
                                    option { "Fashion" }
                                }
                            }
                            div {
                                label { style: "display: block; font-size: var(--text-sm); font-weight: bold; margin-bottom: 0.5rem;", "Vendor / Brand" }
                                input { class: "kh-input", placeholder: "e.g. Kakata Co-op" }
                            }
                        }
                    }

                    div { class: "kh-dash-card-header", "Pricing & Inventory" }
                    div { style: "padding: 2rem; display: grid; grid-template-columns: 1fr 1fr; gap: 1.5rem;",
                        div {
                            label { style: "display: block; font-size: var(--text-sm); font-weight: bold; margin-bottom: 0.5rem;", "Price (LRD)" }
                            input { class: "kh-input", r#type: "number", placeholder: "0.00" }
                        }
                        div {
                            label { style: "display: block; font-size: var(--text-sm); font-weight: bold; margin-bottom: 0.5rem;", "Stock Quantity" }
                            input { class: "kh-input", r#type: "number", placeholder: "0" }
                        }
                    }
                }

                // Media & Actions Column
                div { style: "display: flex; flex-direction: column; gap: 2rem;",
                    div { class: "kh-dash-card",
                        div { class: "kh-dash-card-header", "Product Image" }
                        div { style: "padding: 2rem;",
                            div { 
                                style: "width: 100%; aspect-ratio: 1; border: 2px dashed var(--color-border-subtle); border-radius: var(--radius-xl); display: flex; flex-direction: column; align-items: center; justify-content: center; color: var(--color-text-muted); cursor: pointer; transition: all 0.2s;",
                                span { style: "font-size: 2.5rem; margin-bottom: 1rem;", "📸" }
                                p { style: "font-size: var(--text-sm); font-weight: bold;", "Click to upload" }
                                p { style: "font-size: 0.7rem;", "PNG, JPG up to 10MB" }
                            }
                        }
                    }

                    div { class: "kh-dash-card",
                        div { class: "kh-dash-card-header", "Publish" }
                        div { style: "padding: 2rem; display: flex; flex-direction: column; gap: 1rem;",
                            div { style: "display: flex; justify-content: space-between; align-items: center;",
                                span { style: "font-size: var(--text-sm);", "Status" }
                                span { class: "kh-badge-status success", "Draft" }
                            }
                            div { style: "display: flex; justify-content: space-between; align-items: center;",
                                span { style: "font-size: var(--text-sm);", "Visibility" }
                                span { style: "font-size: var(--text-sm); font-weight: bold;", "Public" }
                            }
                            div { style: "margin-top: 1rem; border-top: 1px solid var(--color-border-subtle); padding-top: 1.5rem;",
                                crate::components::ui::Button { variant: "brand", style_attr: "width: 100%;",
                                    "{button_text}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
