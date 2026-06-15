use crate::components::ui::Button;
use dioxus::prelude::*;

#[component]
pub fn AccountReviews() -> Element {
    rsx! {
        div { class: "kh-animate-fade-in-up",
            div { style: "margin-bottom: 2.5rem;",
                h2 { style: "font-size: 1.75rem; font-weight: var(--font-weight-bold); margin-bottom: 0.5rem;", "My Reviews & Feedback" }
                p { style: "color: var(--color-text-secondary);", "View and manage all the feedback you've shared with the community." }
            }

            div { style: "display: grid; grid-template-columns: 1fr; gap: 2.5rem;",

                // Pending Reviews Section
                div { class: "kh-dash-card",
                    div { class: "kh-dash-card-header",
                        style: "background: var(--color-brand-muted); color: var(--color-brand); border-bottom: 1px solid var(--color-brand-muted);",
                        "Pending Reviews"
                    }
                    div { class: "kh-activity-list",
                        div { class: "kh-activity-item",
                            div { style: "display: flex; gap: 1.5rem; align-items: center;",
                                div {
                                    style: "width: 4rem; height: 4rem; background: var(--color-surface); border-radius: var(--radius-lg); display: flex; align-items: center; justify-content: center; font-size: 1.5rem;",
                                    "🍍"
                                }
                                div {
                                    p { class: "kh-activity-title", "Fresh Gold Pineapples (Bulk)" }
                                    p { class: "kh-activity-sub", "Delivered 2 days ago • From: Nimba Farms" }
                                }
                            }
                            Button { variant: "brand", "Write Review" }
                        }
                    }
                }

                // Published Reviews Section
                div { class: "kh-dash-card",
                    div { class: "kh-dash-card-header", "My Published Reviews" }
                    div { class: "kh-activity-list",
                        div { class: "kh-activity-item", style: "flex-direction: column; align-items: flex-start; gap: 1rem;",
                            div { style: "display: flex; justify-content: space-between; width: 100%; align-items: flex-start;",
                                div { style: "display: flex; gap: 1rem; align-items: center;",
                                    div {
                                        style: "width: 3rem; height: 3rem; background: var(--color-surface); border-radius: var(--radius-md); display: flex; align-items: center; justify-content: center; font-size: 1rem;",
                                        "👜"
                                    }
                                    div {
                                        p { class: "kh-activity-title", "Handmade Leather Tote" }
                                        p { class: "kh-activity-sub", "Reviewed on April 12, 2024" }
                                    }
                                }
                                div { style: "color: #fbbf24; font-size: 1.25rem;", "★★★★★" }
                            }
                            p { style: "margin: 0; color: var(--color-text-secondary); line-height: 1.6;",
                                "The quality of the leather is exceptional. You can tell it's handmade with care. The vendor was very communicative throughout the process."
                            }
                            div { style: "display: flex; gap: 1rem; margin-top: 0.5rem;",
                                Button { variant: "outline", style_attr: "font-size: var(--text-xs); height: 2rem;", "Edit Review" }
                                Button { variant: "outline", style_attr: "font-size: var(--text-xs); height: 2rem; color: #ef4444;", "Delete" }
                            }
                        }

                        div { class: "kh-activity-item", style: "flex-direction: column; align-items: flex-start; gap: 1rem;",
                            div { style: "display: flex; justify-content: space-between; width: 100%; align-items: flex-start;",
                                div { style: "display: flex; gap: 1rem; align-items: center;",
                                    div {
                                        style: "width: 3rem; height: 3rem; background: var(--color-surface); border-radius: var(--radius-md); display: flex; align-items: center; justify-content: center; font-size: 1rem;",
                                        "☕"
                                    }
                                    div {
                                        p { class: "kh-activity-title", "Liberian Robusta Coffee" }
                                        p { class: "kh-activity-sub", "Reviewed on March 28, 2024" }
                                    }
                                }
                                div { style: "color: #fbbf24; font-size: 1.25rem;", "★★★★☆" }
                            }
                            p { style: "margin: 0; color: var(--color-text-secondary); line-height: 1.6;",
                                "Great bold flavor. The packaging was eco-friendly which I loved. Slightly more bitter than the Arabica blend, but overall very fresh."
                            }
                            div { style: "display: flex; gap: 1rem; margin-top: 0.5rem;",
                                Button { variant: "outline", style_attr: "font-size: var(--text-xs); height: 2rem;", "Edit Review" }
                                Button { variant: "outline", style_attr: "font-size: var(--text-xs); height: 2rem; color: #ef4444;", "Delete" }
                            }
                        }
                    }
                }
            }
        }
    }
}
