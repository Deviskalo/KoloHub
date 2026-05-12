pub mod admin_overview;
pub mod products;
pub mod vendor_overview;

use crate::router::Route;
use dioxus::prelude::*;

/// Dashboard layout with persistent side navigation.
#[component]
pub fn DashboardLayout() -> Element {
    rsx! {
        div { class: "kh-dashboard-shell",
            // Persistent Sidebar
            aside { class: "kh-sidebar",
                div { style: "flex: 1; display: flex; flex-direction: column; gap: 0.25rem;",
                    h2 { style: "font-size: var(--text-xs); color: var(--color-text-muted); text-transform: uppercase; letter-spacing: 0.05em; margin: 0 0 0.5rem 1rem;", "Dashboard" }

                    Link {
                        to: Route::VendorOverview {},
                        class: "kh-sidebar-link",
                        active_class: "active",
                        span { "📊" }
                        "Overview"
                    }
                    Link {
                        to: Route::DashboardProducts {},
                        class: "kh-sidebar-link",
                        active_class: "active",
                        span { "📦" }
                        "Products"
                    }
                    // Mock link for now
                    a {
                        href: "#",
                        class: "kh-sidebar-link",
                        span { "📋" }
                        "Orders"
                    }

                    h2 { style: "font-size: var(--text-xs); color: var(--color-text-muted); text-transform: uppercase; letter-spacing: 0.05em; margin: 1.5rem 0 0.5rem 1rem;", "Administration" }
                    Link {
                        to: Route::AdminOverview {},
                        class: "kh-sidebar-link",
                        active_class: "active",
                        span { "⚙️" }
                        "Admin Panel"
                    }
                }

                div { style: "margin-top: auto; padding-top: 1rem; border-top: 1px solid var(--color-border-subtle);",
                    Link {
                        to: Route::Home {},
                        class: "kh-sidebar-link",
                        span { "🌍" }
                        "Go to Marketplace"
                    }
                }
            }

            // Main Content Area inside Dashboard
            main { class: "kh-dashboard-main",
                Outlet::<Route> {}
            }
        }
    }
}
