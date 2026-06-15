use crate::components::ui::Button;
use dioxus::prelude::*;

#[component]
pub fn AccountProfile() -> Element {
    rsx! {
        div { class: "kh-animate-fade-in-up",
            div { class: "kh-profile-card",
                div { class: "kh-profile-header",
                    style: "height: 120px; background: linear-gradient(135deg, var(--color-brand), var(--color-brand-muted)); position: relative;"
                }
                div { class: "kh-profile-body", style: "padding: 0 2.5rem 2.5rem;",
                    div { class: "kh-profile-avatar-wrapper",
                        style: "margin-top: -60px; margin-bottom: 1.5rem; position: relative; width: 120px; height: 120px;",
                        img {
                            class: "kh-profile-avatar",
                            style: "width: 100%; height: 100%; border-radius: 50%; border: 4px solid var(--color-surface); object-fit: cover; box-shadow: var(--shadow-lg);",
                            src: "https://images.unsplash.com/photo-1535713875002-d1d0cf377fde?w=200",
                            alt: "User Avatar"
                        }
                        button {
                            style: "position: absolute; bottom: 5px; right: 5px; background: var(--color-surface); border: 1px solid var(--color-border); border-radius: 50%; width: 2rem; height: 2rem; cursor: pointer; display: flex; align-items: center; justify-content: center; box-shadow: var(--shadow-sm);",
                            "📸"
                        }
                    }

                    div { style: "display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 2.5rem;",
                        div {
                            h2 { style: "font-size: 1.75rem; font-weight: var(--font-weight-bold); margin: 0;", "Johnathan Doe" }
                            p { style: "color: var(--color-text-secondary); margin: 0.25rem 0 0;", "Verified Buyer since Jan 2024 • Monrovia, LR" }
                        }
                        Button { variant: "brand", "Save Profile" }
                    }

                    div { class: "kh-dash-row",
                        // Primary Information Column
                        div { style: "display: flex; flex-direction: column; gap: 2rem;",
                            div { class: "kh-filter-section",
                                h3 { class: "kh-filter-title", "👤 Public Profile" }
                                div { style: "display: flex; flex-direction: column; gap: 1.5rem;",
                                    div {
                                        label { style: "display: block; font-size: var(--text-sm); font-weight: bold; margin-bottom: 0.5rem;", "Full Name" }
                                        input { class: "kh-input", value: "Johnathan Doe" }
                                    }
                                    div {
                                        label { style: "display: block; font-size: var(--text-sm); font-weight: bold; margin-bottom: 0.5rem;", "Bio / About Me" }
                                        textarea {
                                            class: "kh-input",
                                            style: "min-height: 100px; resize: none;",
                                            placeholder: "Tell us about yourself...",
                                            "Fresh produce enthusiast and local marketplace supporter. Always looking for the best harvest drops in Monrovia."
                                        }
                                    }
                                }
                            }

                            div { class: "kh-filter-section",
                                h3 { class: "kh-filter-title", "📞 Contact Details" }
                                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 1.5rem;",
                                    div {
                                        label { style: "display: block; font-size: var(--text-sm); font-weight: bold; margin-bottom: 0.5rem;", "Phone Number" }
                                        input { class: "kh-input", value: "+231 88 000 0000" }
                                    }
                                    div {
                                        label { style: "display: block; font-size: var(--text-sm); font-weight: bold; margin-bottom: 0.5rem;", "Email" }
                                        input { class: "kh-input", value: "j.doe@example.com" }
                                    }
                                }
                            }
                        }

                        // Detailed Settings Column
                        div { style: "display: flex; flex-direction: column; gap: 2rem;",
                            div { class: "kh-filter-section",
                                h3 { class: "kh-filter-title", "📅 Personal Data" }
                                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 1.5rem;",
                                    div {
                                        label { style: "display: block; font-size: var(--text-sm); font-weight: bold; margin-bottom: 0.5rem;", "Date of Birth" }
                                        input { r#type: "date", class: "kh-input", value: "1992-05-12" }
                                    }
                                    div {
                                        label { style: "display: block; font-size: var(--text-sm); font-weight: bold; margin-bottom: 0.5rem;", "Gender" }
                                        select { class: "kh-input",
                                            option { value: "male", selected: true, "Male" }
                                            option { value: "female", "Female" }
                                            option { value: "other", "Other" }
                                        }
                                    }
                                }
                            }

                            div { class: "kh-filter-section",
                                h3 { class: "kh-filter-title", "🌍 Localization" }
                                div { style: "display: flex; flex-direction: column; gap: 1.5rem;",
                                    div {
                                        label { style: "display: block; font-size: var(--text-sm); font-weight: bold; margin-bottom: 0.5rem;", "Interface Language" }
                                        select { class: "kh-input",
                                            option { value: "en", selected: true, "English (Global)" }
                                            option { value: "kr", "Liberian Kreyol" }
                                        }
                                    }
                                    div {
                                        label { style: "display: block; font-size: var(--text-sm); font-weight: bold; margin-bottom: 0.5rem;", "Primary Region" }
                                        select { class: "kh-input",
                                            option { value: "montserrado", selected: true, "Montserrado (Monrovia)" }
                                            option { value: "nimba", "Nimba" }
                                            option { value: "margibi", "Margibi" }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div {
                        style: "margin-top: 3rem; padding-top: 2rem; border-top: 1px solid var(--color-border-subtle); display: flex; justify-content: flex-end; gap: 1rem;",
                        Button { variant: "outline", "Deactivate Account" }
                        Button { variant: "brand", "Save Profile Changes" }
                    }
                }
            }
        }
    }
}
