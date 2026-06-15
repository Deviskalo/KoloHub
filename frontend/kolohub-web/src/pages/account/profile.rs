use dioxus::prelude::*;

#[component]
pub fn AccountProfile() -> Element {
    rsx! {
        div { class: "kh-animate-fade-in-up",
            div { class: "kh-profile-card",
                div { class: "kh-profile-header" }
                div { class: "kh-profile-body",
                    div { class: "kh-profile-avatar-wrapper",
                        img { 
                            class: "kh-profile-avatar", 
                            src: "https://images.unsplash.com/photo-1535713875002-d1d0cf377fde?w=200", 
                            alt: "User Avatar" 
                        }
                    }
                    h2 { style: "font-size: 1.5rem; font-weight: var(--font-weight-bold); margin: 0;", "Johnathan Doe" }
                    p { style: "color: var(--color-text-secondary); margin: 0.25rem 0 2rem;", "Verified Buyer since 2024" }

                    div { style: "display: grid; grid-template-columns: 1fr; gap: 2rem;",
                        div { class: "kh-filter-section",
                            h3 { class: "kh-filter-title", "Contact Information" }
                            div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 1.5rem;",
                                div {
                                    label { style: "display: block; font-size: var(--text-sm); font-weight: bold; margin-bottom: 0.5rem;", "Full Name" }
                                    input { class: "kh-input", value: "Johnathan Doe" }
                                }
                                div {
                                    label { style: "display: block; font-size: var(--text-sm); font-weight: bold; margin-bottom: 0.5rem;", "Phone Number" }
                                    input { class: "kh-input", value: "+231 88 000 0000" }
                                }
                                div {
                                    label { style: "display: block; font-size: var(--text-sm); font-weight: bold; margin-bottom: 0.5rem;", "Email Address" }
                                    input { class: "kh-input", value: "j.doe@example.com" }
                                }
                                div {
                                    label { style: "display: block; font-size: var(--text-sm); font-weight: bold; margin-bottom: 0.5rem;", "Location" }
                                    input { class: "kh-input", value: "Monrovia, Liberia" }
                                }
                            }
                        }

                        div { class: "kh-filter-section",
                            h3 { class: "kh-filter-title", "Preferences" }
                            div { style: "display: flex; flex-direction: column; gap: 1rem;",
                                label { style: "display: flex; align-items: center; gap: 1rem; cursor: pointer;",
                                    input { r#type: "checkbox", checked: true }
                                    span { "Receive SMS updates for order tracking" }
                                }
                                label { style: "display: flex; align-items: center; gap: 1rem; cursor: pointer;",
                                    input { r#type: "checkbox", checked: true }
                                    span { "Notify me about local harvest drops" }
                                }
                            }
                        }

                        div { style: "margin-top: 1rem; display: flex; gap: 1rem;",
                            crate::components::ui::Button { variant: "brand", "Save Profile Changes" }
                            crate::components::ui::Button { variant: "outline", "Reset Password" }
                        }
                    }
                }
            }
        }
    }
}
