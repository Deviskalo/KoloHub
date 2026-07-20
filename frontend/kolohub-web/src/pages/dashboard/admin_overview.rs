use crate::pages::dashboard::vendor_management::{
    get_mock_vendors, ApprovalQueue, VendorDirectory, VendorModel, VendorStatus,
};
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Debug)]
enum AdminTab {
    Overview,
    VendorDirectory,
    ApprovalQueue,
}

#[component]
pub fn AdminOverview() -> Element {
    let mut active_tab = use_signal(|| AdminTab::Overview);
    let mut vendors: Signal<Vec<VendorModel>> = use_signal(get_mock_vendors);

    let pending_count = move || {
        vendors()
            .iter()
            .filter(|v| v.status == VendorStatus::Pending)
            .count()
    };

    let handle_toggle_status = move |(id, new_status): (String, VendorStatus)| {
        vendors.write().iter_mut().for_each(|v| {
            if v.id == id {
                v.status = new_status;
            }
        });
    };

    let handle_approve = move |id: String| {
        vendors.write().iter_mut().for_each(|v| {
            if v.id == id {
                v.status = VendorStatus::Active;
            }
        });
    };

    let handle_reject = move |(id, _reason): (String, String)| {
        // In a real app, send the rejection reason via API
        vendors.write().retain(|v| v.id != id);
    };

    rsx! {
        div {
            // Page Header
            div { class: "kh-dashboard-header",
                div {
                    h1 { class: "kh-dashboard-title", "Platform Administration" }
                    p { style: "color: var(--color-text-muted); margin: 0.25rem 0 0; font-size: var(--text-sm);",
                        "Supervise platform activity, vendors, and total revenue."
                    }
                }
                // Pending badge
                if pending_count() > 0 {
                    div {
                        style: "display: flex; align-items: center; gap: 0.5rem; background: rgba(234, 179, 8, 0.12); border: 1px solid rgba(234, 179, 8, 0.3); padding: 0.5rem 1rem; border-radius: 999px;",
                        span { style: "font-size: var(--text-xs); color: rgb(234, 179, 8); font-weight: var(--font-weight-bold);",
                            "⚠ {pending_count()} application(s) awaiting approval"
                        }
                        button {
                            style: "background: none; border: none; color: rgb(234, 179, 8); font-size: var(--text-xs); cursor: pointer; text-decoration: underline; padding: 0;",
                            onclick: move |_| active_tab.set(AdminTab::ApprovalQueue),
                            "Review Now"
                        }
                    }
                }
            }

            // Tab Nav
            div { class: "kh-tabs",
                button {
                    class: if active_tab() == AdminTab::Overview { "kh-tab active" } else { "kh-tab" },
                    onclick: move |_| active_tab.set(AdminTab::Overview),
                    "📊  Overview"
                }
                button {
                    class: if active_tab() == AdminTab::VendorDirectory { "kh-tab active" } else { "kh-tab" },
                    onclick: move |_| active_tab.set(AdminTab::VendorDirectory),
                    "🏪  Vendor Directory"
                }
                button {
                    class: if active_tab() == AdminTab::ApprovalQueue { "kh-tab active" } else { "kh-tab" },
                    onclick: move |_| active_tab.set(AdminTab::ApprovalQueue),
                    "📋  Approval Queue"
                    if pending_count() > 0 {
                        span {
                            style: "margin-left: 0.5rem; background: rgb(234,179,8); color: #000; border-radius: 999px; font-size: 0.65rem; font-weight: bold; padding: 0.1rem 0.45rem;",
                            "{pending_count()}"
                        }
                    }
                }
            }

            // Tab Content
            div { class: "kh-tab-content",
                match active_tab() {
                    AdminTab::Overview => {
                        rsx! {
                            // KPI Stat Cards
                            div { class: "kh-stats-grid",
                                div { class: "kh-stat-card",
                                    p { class: "kh-stat-label", "Total Platform Users" }
                                    p { class: "kh-stat-val", "12,540" }
                                    span { class: "kh-stat-trend up", "↑ 18.0% vs last month" }
                                }
                                div { class: "kh-stat-card",
                                    p { class: "kh-stat-label", "Active Vendors" }
                                    p { class: "kh-stat-val", "{vendors().iter().filter(|v| v.status == VendorStatus::Active).count()}" }
                                    span { class: "kh-stat-trend up", "↑ 12.1% vs last month" }
                                }
                                div { class: "kh-stat-card",
                                    p { class: "kh-stat-label", "Global Revenue (LRD)" }
                                    p { class: "kh-stat-val", "450.2M" }
                                    span { class: "kh-stat-trend up", "↑ 9.4% vs last month" }
                                }
                                div { class: "kh-stat-card",
                                    p { class: "kh-stat-label", "Conversion Rate" }
                                    p { class: "kh-stat-val", "3.8%" }
                                    span { class: "kh-stat-trend down", "↓ 0.3% vs last month" }
                                }
                            }

                            // Charts and Activity
                            div { class: "kh-dash-row",
                                div { class: "kh-dash-card",
                                    h2 { class: "kh-dash-card-header", "Platform Revenue Timeline" }
                                    div { class: "kh-dash-chart-placeholder",
                                        "[ Revenue Chart · Backend Integration Pending ]"
                                    }
                                }
                                div { class: "kh-dash-card",
                                    h2 { class: "kh-dash-card-header", "Top Vendors by Revenue" }
                                    div { class: "kh-activity-list",
                                        for v in vendors().iter().filter(|v| v.status == VendorStatus::Active).take(5).cloned().collect::<Vec<_>>() {
                                            div { class: "kh-activity-item",
                                                div { class: "kh-activity-info",
                                                    p { class: "kh-activity-title", "{v.name}" }
                                                    p { class: "kh-activity-sub", "{v.county} · {v.category}" }
                                                }
                                                p { style: "font-weight: var(--font-weight-bold); white-space: nowrap;",
                                                    if v.sales >= 1_000_000.0 {
                                                        "LRD {(v.sales / 1_000_000.0):.1}M"
                                                    } else {
                                                        "LRD {(v.sales / 1000.0):.0}k"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // Suspended Vendor Alert
                            {
                                let suspended: Vec<VendorModel> = vendors().iter().filter(|v| v.status == VendorStatus::Suspended).cloned().collect();
                                if !suspended.is_empty() {
                                    rsx! {
                                        div {
                                            style: "background: rgba(239,68,68,0.07); border: 1px solid rgba(239,68,68,0.25); border-radius: var(--radius-md); padding: 1rem 1.25rem; margin-top: 1.5rem;",
                                            p { style: "margin: 0 0 0.5rem; font-weight: var(--font-weight-semibold); color: rgb(239,68,68);",
                                                "⚠ {suspended.len()} vendor(s) currently suspended"
                                            }
                                            for v in suspended {
                                                p { style: "margin: 0.15rem 0; font-size: var(--text-xs); color: var(--color-text-muted);",
                                                    "• {v.name} ({v.county}) · suspended"
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    rsx! {}
                                }
                            }
                        }
                    }
                    AdminTab::VendorDirectory => {
                        rsx! {
                            VendorDirectory {
                                vendors: vendors,
                                on_toggle_status: handle_toggle_status,
                            }
                        }
                    }
                    AdminTab::ApprovalQueue => {
                        rsx! {
                            ApprovalQueue {
                                vendors: vendors,
                                on_approve: handle_approve,
                                on_reject: handle_reject,
                            }
                        }
                    }
                }
            }
        }
    }
}
