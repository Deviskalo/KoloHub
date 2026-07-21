use crate::pages::dashboard::vendor_management::{
    get_mock_vendors, ApprovalQueue, VendorDirectory, VendorModel, VendorStatus,
};
use dioxus::prelude::*;

// ── Chart data ──────────────────────────────────────────────────────────────
const REVENUE_DATA: &[f64] = &[1.8, 2.7, 2.2, 3.8, 3.2, 4.5, 4.1, 5.3, 4.8, 6.0, 5.5, 7.2];
const REVENUE_LABELS: &[&str] = &[
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];
const USERS_DATA: &[f64] = &[
    7200.0, 8050.0, 8900.0, 9400.0, 10100.0, 10900.0, 11300.0, 11800.0, 12050.0, 12200.0, 12400.0,
    12540.0,
];

// ── Activity feed items ──────────────────────────────────────────────────────
struct ActivityEntry {
    icon: &'static str,
    title: &'static str,
    sub: &'static str,
    time: &'static str,
    tag: &'static str,
    tag_class: &'static str,
}

fn mock_activity() -> Vec<ActivityEntry> {
    vec![
        ActivityEntry {
            icon: "🔐",
            title: "Admin login from new device",
            sub: "IP 197.210.64.3 · Monrovia, LR",
            time: "2 min ago",
            tag: "Security",
            tag_class: "danger",
        },
        ActivityEntry {
            icon: "🛒",
            title: "High-value purchase — LRD 2.4M",
            sub: "Order #KH-29847 by James T. · Monrovia Electronics",
            time: "11 min ago",
            tag: "Purchase",
            tag_class: "success",
        },
        ActivityEntry {
            icon: "🏪",
            title: "New vendor approved",
            sub: "Gbarnga Crafts Hub · Bong County",
            time: "34 min ago",
            tag: "Vendor",
            tag_class: "info",
        },
        ActivityEntry {
            icon: "👤",
            title: "New user signup",
            sub: "mercy.kollie@gmail.com · Signed up via Google",
            time: "1 hr ago",
            tag: "Signup",
            tag_class: "warning",
        },
        ActivityEntry {
            icon: "🛒",
            title: "Bulk order placed — LRD 890k",
            sub: "Order #KH-29801 · Fresh Farm Produce",
            time: "2 hr ago",
            tag: "Purchase",
            tag_class: "success",
        },
        ActivityEntry {
            icon: "⚠",
            title: "Failed payment attempt",
            sub: "3 attempts · Card ending 4412 · Karmon's Boutique",
            time: "3 hr ago",
            tag: "Security",
            tag_class: "danger",
        },
        ActivityEntry {
            icon: "👤",
            title: "5 new user signups",
            sub: "Batch registration via referral campaign",
            time: "4 hr ago",
            tag: "Signup",
            tag_class: "warning",
        },
        ActivityEntry {
            icon: "🏪",
            title: "Vendor suspended",
            sub: "Coastal Seafood Co. · Terms violation",
            time: "6 hr ago",
            tag: "Vendor",
            tag_class: "danger",
        },
    ]
}

// ── SVG helpers ──────────────────────────────────────────────────────────────
fn make_line_path(data: &[f64], w: f64, h: f64) -> String {
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let range = (max - min).max(0.0001);
    let step = w / (data.len().saturating_sub(1)) as f64;
    data.iter()
        .enumerate()
        .map(|(i, v)| {
            let x = i as f64 * step;
            let y = h - ((v - min) / range * (h * 0.85)) - (h * 0.075);
            if i == 0 {
                format!("M {x:.1} {y:.1}")
            } else {
                format!("L {x:.1} {y:.1}")
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn make_area_path(data: &[f64], w: f64, h: f64) -> String {
    format!(
        "{} L {:.1} {h:.1} L 0 {h:.1} Z",
        make_line_path(data, w, h),
        w
    )
}

fn sparkline_path(data: &[f64]) -> String {
    make_line_path(data, 80.0, 30.0)
}

// ── Tab enum ─────────────────────────────────────────────────────────────────
#[derive(Clone, PartialEq, Debug)]
enum AdminTab {
    Overview,
    VendorDirectory,
    ApprovalQueue,
}

// ── Root component ────────────────────────────────────────────────────────────
#[component]
pub fn AdminOverview() -> Element {
    let mut active_tab = use_signal(|| AdminTab::Overview);
    let mut vendors: Signal<Vec<VendorModel>> = use_signal(get_mock_vendors);
    let mut active_chart = use_signal(|| "revenue"); // "revenue" | "users"

    let pending_count = move || {
        vendors()
            .iter()
            .filter(|v| v.status == VendorStatus::Pending)
            .count()
    };
    let active_count = move || {
        vendors()
            .iter()
            .filter(|v| v.status == VendorStatus::Active)
            .count()
    };

    let handle_toggle = move |(id, status): (String, VendorStatus)| {
        vendors.write().iter_mut().for_each(|v| {
            if v.id == id {
                v.status = status;
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
        vendors.write().retain(|v| v.id != id);
    };

    // Pre-compute chart paths before rsx!
    let chart_data: &[f64] = if active_chart() == "users" {
        USERS_DATA
    } else {
        REVENUE_DATA
    };
    let line_path = make_line_path(chart_data, 560.0, 180.0);
    let area_path = make_area_path(chart_data, 560.0, 180.0);
    let chart_title = if active_chart() == "revenue" {
        "📈 Revenue Timeline"
    } else {
        "👥 User Growth"
    };

    // Pre-compute dot positions for the SVG chart
    let chart_dots: Vec<(f64, f64)> = {
        let max = chart_data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let min = chart_data.iter().cloned().fold(f64::INFINITY, f64::min);
        let range = (max - min).max(0.0001);
        let step = 560.0 / (chart_data.len().saturating_sub(1)) as f64;
        chart_data
            .iter()
            .enumerate()
            .map(|(i, v)| {
                let cx = i as f64 * step;
                let cy = 180.0 - ((v - min) / range * (180.0 * 0.85)) - (180.0 * 0.075);
                (cx, cy)
            })
            .collect()
    };

    // Pre-compute bar chart values
    let bar_values: Vec<(f64, String)> = {
        let max = USERS_DATA.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        USERS_DATA
            .iter()
            .map(|v| {
                let h = v / max * 100.0;
                let label = format!("{:.1}k", v / 1000.0);
                (h, label)
            })
            .collect()
    };

    // Pre-compute top vendor display strings
    let top_vendors: Vec<(String, String, String)> = vendors()
        .iter()
        .filter(|v| v.status == VendorStatus::Active)
        .take(5)
        .map(|v| {
            let sales_str = if v.sales >= 1_000_000.0 {
                format!("LRD {:.1}M", v.sales / 1_000_000.0)
            } else {
                format!("LRD {:.0}k", v.sales / 1000.0)
            };
            (
                v.name.clone(),
                format!("{} · {}", v.county, v.category),
                sales_str,
            )
        })
        .collect();

    let suspended_count = vendors()
        .iter()
        .filter(|v| v.status == VendorStatus::Suspended)
        .count();

    rsx! {
        div {
            // ── Page header ───────────────────────────────────────────────
            div { class: "kh-dashboard-header",
                div {
                    h1 { class: "kh-dashboard-title", "Platform Administration" }
                    p { style: "color: var(--color-text-muted); margin: 0.25rem 0 0; font-size: var(--text-sm);",
                        "Supervise platform activity, vendors, and total revenue."
                    }
                }
                if pending_count() > 0 {
                    div { class: "kh-admin-alert-pill",
                        span { "⚠ {pending_count()} application(s) awaiting approval" }
                        button { onclick: move |_| active_tab.set(AdminTab::ApprovalQueue),
                            "Review Now →"
                        }
                    }
                }
            }

            // ── Tab nav ───────────────────────────────────────────────────
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
                        span { class: "kh-tab-badge", "{pending_count()}" }
                    }
                }
            }

            // ── Tab content ───────────────────────────────────────────────
            div { class: "kh-tab-content",
                if active_tab() == AdminTab::Overview {
                    // ── KPI Cards ────────────────────────────────────────
                    div { class: "kh-admin-kpi-row",
                        KpiCard {
                            label: "Total Users",
                            value: "12,540".to_string(),
                            trend: "+18.0%",
                            up: true,
                            sparkline: USERS_DATA,
                        }
                        KpiCard {
                            label: "Active Vendors",
                            value: format!("{}", active_count()),
                            trend: "+12.1%",
                            up: true,
                            sparkline: REVENUE_DATA,
                        }
                        KpiCard {
                            label: "Global Revenue",
                            value: "LRD 450.2M".to_string(),
                            trend: "+9.4%",
                            up: true,
                            sparkline: REVENUE_DATA,
                        }
                        KpiCard {
                            label: "Conversion Rate",
                            value: "3.8%".to_string(),
                            trend: "−0.3%",
                            up: false,
                            sparkline: USERS_DATA,
                        }
                        KpiCard {
                            label: "Avg. Order Value",
                            value: "LRD 38.4k".to_string(),
                            trend: "+5.7%",
                            up: true,
                            sparkline: REVENUE_DATA,
                        }
                    }

                    // ── Chart + Activity Feed ────────────────────────────
                    div { class: "kh-admin-dash-grid",
                        // SVG Line Chart
                        div { class: "kh-dash-card",
                            div { style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 1rem; padding-bottom: 0.75rem; border-bottom: 1px solid var(--color-border-subtle);",
                                h2 { style: "margin: 0; font-size: var(--text-lg); font-weight: var(--font-weight-bold); color: var(--color-text-primary);",
                                    "{chart_title}"
                                }
                                div { style: "display: flex; gap: 0.5rem;",
                                    button {
                                        class: if active_chart() == "revenue" { "kh-chart-toggle active" } else { "kh-chart-toggle" },
                                        onclick: move |_| active_chart.set("revenue"),
                                        "Revenue"
                                    }
                                    button {
                                        class: if active_chart() == "users" { "kh-chart-toggle active" } else { "kh-chart-toggle" },
                                        onclick: move |_| active_chart.set("users"),
                                        "Users"
                                    }
                                }
                            }
                            div { class: "kh-chart-container",
                                svg {
                                    view_box: "0 0 560 180",
                                    preserve_aspect_ratio: "none",
                                    style: "width: 100%; height: 180px; overflow: visible;",
                                    defs {
                                        linearGradient {
                                            id: "chartGrad",
                                            x1: "0",
                                            y1: "0",
                                            x2: "0",
                                            y2: "1",
                                            stop {
                                                offset: "0%",
                                                stop_color: "var(--color-brand)",
                                                stop_opacity: "0.3",
                                            }
                                            stop {
                                                offset: "100%",
                                                stop_color: "var(--color-brand)",
                                                stop_opacity: "0.0",
                                            }
                                        }
                                    }
                                    for i in 0..5u32 {
                                        line {
                                            key: "{i}",
                                            x1: "0",
                                            y1: "{i * 36}",
                                            x2: "560",
                                            y2: "{i * 36}",
                                            stroke: "var(--color-border-subtle)",
                                            stroke_width: "1",
                                            stroke_dasharray: "4 4",
                                        }
                                    }
                                    path {
                                        d: "{area_path}",
                                        fill: "url(#chartGrad)",
                                    }
                                    path {
                                        d: "{line_path}",
                                        fill: "none",
                                        stroke: "var(--color-brand)",
                                        stroke_width: "2.5",
                                        stroke_linejoin: "round",
                                        stroke_linecap: "round",
                                    }
                                    for (cx , cy) in chart_dots.iter() {
                                        circle {
                                            key: "{cx}",
                                            cx: "{cx:.1}",
                                            cy: "{cy:.1}",
                                            r: "4",
                                            fill: "var(--color-surface-elevated)",
                                            stroke: "var(--color-brand)",
                                            stroke_width: "2",
                                        }
                                    }
                                }
                                div { class: "kh-chart-labels",
                                    for label in REVENUE_LABELS {
                                        span { "{label}" }
                                    }
                                }
                            }
                        }

                        // Activity Feed
                        div { class: "kh-dash-card",
                            h2 { class: "kh-dash-card-header", "⚡ Recent Platform Activity" }
                            div { class: "kh-activity-feed",
                                for entry in mock_activity() {
                                    div { class: "kh-feed-entry",
                                        div { class: "kh-feed-icon-col",
                                            div { class: "kh-feed-dot kh-feed-dot-{entry.tag_class}" }
                                        }
                                        div { class: "kh-feed-body",
                                            div { style: "display: flex; justify-content: space-between; align-items: flex-start; gap: 0.5rem;",
                                                div {
                                                    p { class: "kh-feed-title",
                                                        span { style: "margin-right: 0.4rem;",
                                                            "{entry.icon}"
                                                        }
                                                        "{entry.title}"
                                                    }
                                                    p { class: "kh-feed-sub", "{entry.sub}" }
                                                }
                                                div { style: "display: flex; flex-direction: column; align-items: flex-end; gap: 0.3rem; white-space: nowrap;",
                                                    span { class: "kh-badge {entry.tag_class}",
                                                        "{entry.tag}"
                                                    }
                                                    span { class: "kh-feed-time", "{entry.time}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // ── Bar Chart + Top Vendors ────────────────────────────
                    div {
                        class: "kh-admin-dash-grid",
                        style: "margin-top: 1.5rem;",
                        div { class: "kh-dash-card",
                            h2 { class: "kh-dash-card-header", "📊 Monthly Registered Users" }
                            div { class: "kh-bar-chart",
                                for (bar_h , label) in bar_values.iter() {
                                    div { class: "kh-bar-col",
                                        div { class: "kh-bar-value", "{label}" }
                                        div {
                                            class: "kh-bar",
                                            style: "height: {bar_h:.0}px;",
                                        }
                                    }
                                }
                            }
                            div { class: "kh-chart-labels",
                                for l in REVENUE_LABELS {
                                    span { "{l}" }
                                }
                            }
                        }

                        div { class: "kh-dash-card",
                            h2 { class: "kh-dash-card-header", "🏆 Top Active Vendors" }
                            div { class: "kh-activity-list",
                                for (name , meta , sales) in top_vendors.iter() {
                                    div { class: "kh-activity-item",
                                        div { class: "kh-activity-info",
                                            p { class: "kh-activity-title", "{name}" }
                                            p { class: "kh-activity-sub", "{meta}" }
                                        }
                                        p { style: "font-weight: var(--font-weight-bold); white-space: nowrap; color: var(--color-brand);",
                                            "{sales}"
                                        }
                                    }
                                }
                            }
                            if suspended_count > 0 {
                                div { style: "margin-top: 1rem; padding: 0.75rem 1rem; background: rgba(239,68,68,0.07); border: 1px solid rgba(239,68,68,0.2); border-radius: var(--radius-md);",
                                    p { style: "margin: 0; font-size: var(--text-xs); color: rgb(239,68,68); font-weight: var(--font-weight-semibold);",
                                        "⚠ {suspended_count} vendor(s) currently suspended"
                                    }
                                }
                            }
                        }
                    }
                } else if active_tab() == AdminTab::VendorDirectory {
                    VendorDirectory { vendors, on_toggle_status: handle_toggle }
                } else {
                    ApprovalQueue {
                        vendors,
                        on_approve: handle_approve,
                        on_reject: handle_reject,
                    }
                }
            }
        }
    }
}

// ── KPI Card component ────────────────────────────────────────────────────────
#[component]
fn KpiCard(
    label: &'static str,
    value: String,
    trend: &'static str,
    up: bool,
    sparkline: &'static [f64],
) -> Element {
    let spark = sparkline_path(sparkline);
    let trend_class = if up { "up" } else { "down" };

    rsx! {
        div { class: "kh-stat-card kh-kpi-card",
            div { style: "flex: 1;",
                p { class: "kh-stat-label", "{label}" }
                p { class: "kh-stat-val", "{value}" }
                span { class: "kh-stat-trend {trend_class}", "{trend}" }
            }
            svg {
                view_box: "0 0 80 30",
                style: "width: 80px; height: 30px; flex-shrink: 0;",
                path {
                    d: "{spark}",
                    fill: "none",
                    stroke: if up { "rgb(16,185,129)" } else { "rgb(239,68,68)" },
                    stroke_width: "2",
                    stroke_linejoin: "round",
                    stroke_linecap: "round",
                }
            }
        }
    }
}
