use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct VendorModel {
    pub id: String,
    pub name: String,
    pub owner: String,
    pub email: String,
    pub phone: String,
    pub county: String,
    pub category: String,
    pub joined: String,
    pub sales: f64,
    pub status: VendorStatus,
    pub description: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VendorStatus {
    Active,
    Suspended,
    Pending,
}

impl VendorStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "Active",
            Self::Suspended => "Suspended",
            Self::Pending => "Pending",
        }
    }
}

pub fn get_mock_vendors() -> Vec<VendorModel> {
    vec![
        VendorModel {
            id: "V001".to_string(),
            name: "Monrovia Electronics".to_string(),
            owner: "John Kollie".to_string(),
            email: "john.kollie@gmail.com".to_string(),
            phone: "+231 88 567 4321".to_string(),
            county: "Montserrado".to_string(),
            category: "Electronics".to_string(),
            joined: "2024-01-15".to_string(),
            sales: 14500000.0,
            status: VendorStatus::Active,
            description: "Leading reseller of consumer electronics, smartphones, and accessories in Central Monrovia.".to_string(),
        },
        VendorModel {
            id: "V002".to_string(),
            name: "Karmon's Boutique".to_string(),
            owner: "Princess Karmon".to_string(),
            email: "princess.karmon@boutique.lr".to_string(),
            phone: "+231 77 123 4567".to_string(),
            county: "Montserrado".to_string(),
            category: "Fashion".to_string(),
            joined: "2024-03-20".to_string(),
            sales: 8200000.0,
            status: VendorStatus::Active,
            description: "High-end authentic Liberian designs, Lappa suits, and traditional African wedding wear.".to_string(),
        },
        VendorModel {
            id: "V003".to_string(),
            name: "Fresh Farm Produce".to_string(),
            owner: "Joseph Sherman".to_string(),
            email: "joseph.sherman@farm.lr".to_string(),
            phone: "+231 88 987 6543".to_string(),
            county: "Lofa".to_string(),
            category: "Groceries".to_string(),
            joined: "2024-02-10".to_string(),
            sales: 4100000.0,
            status: VendorStatus::Active,
            description: "Sourcing country rice, bitter balls, cassava leaves, and fresh vegetables directly from Lofa farm cooperatives.".to_string(),
        },
        VendorModel {
            id: "V004".to_string(),
            name: "Nimba Cocoa Ventures".to_string(),
            owner: "Martha Cooper".to_string(),
            email: "martha.cooper@nimba.lr".to_string(),
            phone: "+231 77 456 1234".to_string(),
            county: "Nimba".to_string(),
            category: "Agribusiness".to_string(),
            joined: "2024-05-12".to_string(),
            sales: 1200000.0,
            status: VendorStatus::Suspended,
            description: "Specialized in raw and roasted Nimba cocoa beans, custom cocoa powders, and local chocolates.".to_string(),
        },
        VendorModel {
            id: "V005".to_string(),
            name: "Bassa Fishing Corp".to_string(),
            owner: "Emmanuel Tarr".to_string(),
            email: "emmanuel.tarr@bassa.lr".to_string(),
            phone: "+231 88 333 4444".to_string(),
            county: "Grand Bassa".to_string(),
            category: "Groceries".to_string(),
            joined: "2026-07-15".to_string(),
            sales: 0.0,
            status: VendorStatus::Pending,
            description: "Looking to provide fresh and dried seafood delivery within Buchanan and Monrovia with cold-room storage.".to_string(),
        },
        VendorModel {
            id: "V006".to_string(),
            name: "Gbarnga Crafts Hub".to_string(),
            owner: "Sia Mulbah".to_string(),
            email: "sia.mulbah@crafts.lr".to_string(),
            phone: "+231 77 888 9999".to_string(),
            county: "Bong".to_string(),
            category: "Home & Living".to_string(),
            joined: "2026-07-18".to_string(),
            sales: 0.0,
            status: VendorStatus::Pending,
            description: "Local artisan cooperative making hand-woven cane chairs, baskets, and carved sculptures.".to_string(),
        },
    ]
}

#[component]
pub fn VendorDirectory(
    vendors: Signal<Vec<VendorModel>>,
    on_toggle_status: Callback<(String, VendorStatus)>,
) -> Element {
    let mut search_query = use_signal(|| "".to_string());
    let mut status_filter = use_signal(|| "All".to_string());
    let mut detail_vendor: Signal<Option<VendorModel>> = use_signal(|| None);

    let filtered_vendors = move || {
        vendors()
            .iter()
            .filter(|v| {
                if v.status == VendorStatus::Pending {
                    return false; // Directory only shows Active/Suspended
                }
                let matches_search = v
                    .name
                    .to_lowercase()
                    .contains(&search_query().to_lowercase())
                    || v.owner
                        .to_lowercase()
                        .contains(&search_query().to_lowercase());

                let matches_status = match status_filter().as_str() {
                    "All" => true,
                    "Active" => v.status == VendorStatus::Active,
                    "Suspended" => v.status == VendorStatus::Suspended,
                    _ => true,
                };

                matches_search && matches_status
            })
            .cloned()
            .collect::<Vec<_>>()
    };

    rsx! {
        div {
            // Controls Bar
            div { class: "kh-filter-bar",
                input {
                    class: "kh-admin-input",
                    placeholder: "Search vendors or owners...",
                    value: "{search_query}",
                    oninput: move |e| search_query.set(e.value()),
                }
                select {
                    class: "kh-admin-select",
                    value: "{status_filter}",
                    onchange: move |e| status_filter.set(e.value()),
                    option { value: "All", "All Statuses" }
                    option { value: "Active", "Active Only" }
                    option { value: "Suspended", "Suspended Only" }
                }
            }

            // Vendor Table
            div { class: "kh-admin-table-wrapper",
                table { class: "kh-admin-table",
                    thead {
                        tr {
                            th { "Vendor ID" }
                            th { "Shop Name" }
                            th { "Owner" }
                            th { "County" }
                            th { "Category" }
                            th { "Sales (LRD)" }
                            th { "Status" }
                            th { style: "text-align: right;", "Actions" }
                        }
                    }
                    tbody {
                        if filtered_vendors().is_empty() {
                            tr {
                                td {
                                    colspan: 8,
                                    style: "text-align: center; color: var(--color-text-muted); padding: 2rem;",
                                    "No vendors found matching filters."
                                }
                            }
                        } else {
                            for v in filtered_vendors() {
                                tr { key: "{v.id}",
                                    td { style: "font-family: monospace; font-weight: bold;",
                                        "{v.id}"
                                    }
                                    td {
                                        {
                                            let v_link = v.clone();
                                            rsx! {
                                                a {
                                                    href: "#",
                                                    style: "color: var(--color-brand); font-weight: var(--font-weight-semibold); text-decoration: none;",
                                                    onclick: move |e| {
                                                        e.prevent_default();
                                                        detail_vendor.set(Some(v_link.clone()));
                                                    },
                                                    "{v.name}"
                                                }
                                            }
                                        }
                                    }
                                    td { "{v.owner}" }
                                    td { "{v.county}" }
                                    td { "{v.category}" }
                                    td {
                                        {
                                            let formatted = if v.sales >= 1_000_000.0 {
                                                format!("{:.1}M", v.sales / 1_000_000.0)
                                            } else {
                                                format!("{:.1}k", v.sales / 1000.0)
                                            };
                                            rsx! { "{formatted}" }
                                        }
                                    }
                                    td {
                                        if v.status == VendorStatus::Active {
                                            span { class: "kh-badge success", "Active" }
                                        } else if v.status == VendorStatus::Suspended {
                                            span { class: "kh-badge danger", "Suspended" }
                                        } else {
                                            span { class: "kh-badge warning", "Pending" }
                                        }
                                    }
                                    td { style: "text-align: right; display: flex; gap: 0.5rem; justify-content: flex-end;",
                                        {
                                            let tid = v.id.clone();
                                            let is_active = v.status == VendorStatus::Active;
                                            rsx! {
                                                if is_active {
                                                    button {
                                                        class: "kh-btn-outline",
                                                        style: "padding: 0.35rem 0.7rem; font-size: var(--text-xs); border-color: rgb(239, 68, 68); color: rgb(239, 68, 68);",
                                                        onclick: move |_| on_toggle_status.call((tid.clone(), VendorStatus::Suspended)),
                                                        "Suspend"
                                                    }
                                                } else {
                                                    button {
                                                        class: "kh-btn-outline",
                                                        style: "padding: 0.35rem 0.7rem; font-size: var(--text-xs); border-color: rgb(16, 185, 129); color: rgb(16, 185, 129);",
                                                        onclick: move |_| on_toggle_status.call((tid.clone(), VendorStatus::Active)),
                                                        "Activate"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Vendor Detail Modal
            if let Some(v) = detail_vendor() {
                div {
                    class: "kh-modal-backdrop",
                    onclick: move |_| detail_vendor.set(None),
                    div {
                        class: "kh-modal",
                        onclick: move |e| e.stop_propagation(),
                        div { class: "kh-modal-header",
                            h3 { class: "kh-modal-title", "Vendor Details" }
                            button {
                                class: "kh-modal-close-btn",
                                onclick: move |_| detail_vendor.set(None),
                                "×"
                            }
                        }

                        div { style: "margin-bottom: 1.5rem; text-align: center; border-bottom: 1px solid var(--color-border-subtle); padding-bottom: 1rem;",
                            h4 { style: "margin: 0; font-size: 1.2rem; color: var(--color-text-primary);",
                                "{v.name}"
                            }
                            p { style: "margin: 0.25rem 0 0; font-size: var(--text-xs); color: var(--color-text-muted);",
                                "Joined {v.joined} • ID: {v.id}"
                            }
                        }

                        div { class: "kh-info-grid",
                            div { class: "kh-info-block",
                                span { class: "kh-info-label", "Owner" }
                                span { class: "kh-info-value", "{v.owner}" }
                            }
                            div { class: "kh-info-block",
                                span { class: "kh-info-label", "Category" }
                                span { class: "kh-info-value", "{v.category}" }
                            }
                            div { class: "kh-info-block",
                                span { class: "kh-info-label", "Email Address" }
                                span { class: "kh-info-value", "{v.email}" }
                            }
                            div { class: "kh-info-block",
                                span { class: "kh-info-label", "Phone Number" }
                                span { class: "kh-info-value", "{v.phone}" }
                            }
                            div { class: "kh-info-block",
                                span { class: "kh-info-label", "County Location" }
                                span { class: "kh-info-value", "{v.county}" }
                            }
                            div { class: "kh-info-block",
                                span { class: "kh-info-label", "Total Sales" }
                                {
                                    let sales_str = format!("LRD {:.2}", v.sales);
                                    rsx! {
                                        span { class: "kh-info-value", "{sales_str}" }
                                    }
                                }
                            }
                        }

                        div { style: "margin-bottom: 1.75rem;",
                            span { class: "kh-info-label", "Shop Description" }
                            p { style: "margin: 0.35rem 0 0; font-size: var(--text-sm); line-height: 1.5; color: var(--color-text-secondary);",
                                "{v.description}"
                            }
                        }

                        div { class: "kh-action-row",
                            crate::components::ui::Button {
                                variant: "outline",
                                onclick: move |_| detail_vendor.set(None),
                                "Close Details"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ApprovalQueue(
    vendors: Signal<Vec<VendorModel>>,
    on_approve: Callback<String>,
    on_reject: Callback<(String, String)>,
) -> Element {
    let mut inspect_vendor: Signal<Option<VendorModel>> = use_signal(|| None);
    let mut rejection_reason = use_signal(|| "".to_string());
    let mut show_reject_confirm = use_signal(|| false);

    let pending_vendors = move || {
        vendors()
            .iter()
            .filter(|v| v.status == VendorStatus::Pending)
            .cloned()
            .collect::<Vec<_>>()
    };

    rsx! {
        div {
            h3 { style: "font-size: var(--text-md); margin-bottom: 1rem; color: var(--color-text-muted);",
                "Review pending shop applications for the KoloHub platform."
            }

            // Pending Table
            div { class: "kh-admin-table-wrapper",
                table { class: "kh-admin-table",
                    thead {
                        tr {
                            th { "Location" }
                            th { "Shop Name" }
                            th { "Owner" }
                            th { "Category" }
                            th { "Details" }
                            th { style: "text-align: right;", "Actions" }
                        }
                    }
                    tbody {
                        if pending_vendors().is_empty() {
                            tr {
                                td {
                                    colspan: 6,
                                    style: "text-align: center; color: var(--color-text-muted); padding: 3rem;",
                                    "✨ No pending vendor registrations. All caught up!"
                                }
                            }
                        } else {
                            for v in pending_vendors() {
                                tr { key: "{v.id}",
                                    td { style: "font-weight: bold; color: var(--color-brand);",
                                        "{v.county}"
                                    }
                                    td { style: "font-weight: var(--font-weight-semibold);",
                                        "{v.name}"
                                    }
                                    td { "{v.owner}" }
                                    td { "{v.category}" }
                                    td {
                                        {
                                            let v1 = v.clone();
                                            rsx! {
                                                button {
                                                    class: "kh-btn-outline",
                                                    style: "padding: 0.3rem 0.6rem; font-size: var(--text-xs);",
                                                    onclick: move |_| {
                                                        inspect_vendor.set(Some(v1.clone()));
                                                        show_reject_confirm.set(false);
                                                        rejection_reason.set("".to_string());
                                                    },
                                                    "Inspect"
                                                }
                                            }
                                        }
                                    }
                                    td { style: "text-align: right; display: flex; gap: 0.5rem; justify-content: flex-end;",
                                        {
                                            let approve_id = v.id.clone();
                                            let v2 = v.clone();
                                            rsx! {
                                                button {
                                                    class: "kh-btn-outline",
                                                    style: "padding: 0.35rem 0.75rem; font-size: var(--text-xs); border-color: rgb(16, 185, 129); color: rgb(16, 185, 129);",
                                                    onclick: move |_| {
                                                        on_approve.call(approve_id.clone());
                                                    },
                                                    "Approve"
                                                }
                                                button {
                                                    class: "kh-btn-outline",
                                                    style: "padding: 0.35rem 0.75rem; font-size: var(--text-xs); border-color: rgb(239, 68, 68); color: rgb(239, 68, 68);",
                                                    onclick: move |_| {
                                                        inspect_vendor.set(Some(v2.clone()));
                                                        show_reject_confirm.set(true);
                                                    },
                                                    "Reject"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Inspection and Approval Modal
            if let Some(v) = inspect_vendor() {
                div {
                    class: "kh-modal-backdrop",
                    onclick: move |_| {
                        inspect_vendor.set(None);
                        show_reject_confirm.set(false);
                    },
                    div {
                        class: "kh-modal",
                        onclick: move |e| e.stop_propagation(),
                        div { class: "kh-modal-header",
                            h3 { class: "kh-modal-title",
                                if show_reject_confirm() {
                                    "Confirm Rejection"
                                } else {
                                    "Review Application"
                                }
                            }
                            button {
                                class: "kh-modal-close-btn",
                                onclick: move |_| {
                                    inspect_vendor.set(None);
                                    show_reject_confirm.set(false);
                                },
                                "×"
                            }
                        }

                        if !show_reject_confirm() {
                            div { style: "margin-bottom: 1.5rem; text-align: center; border-bottom: 1px solid var(--color-border-subtle); padding-bottom: 1rem;",
                                span {
                                    class: "kh-badge info",
                                    style: "margin-bottom: 0.5rem;",
                                    "PENDING REVIEW"
                                }
                                h4 { style: "margin: 0; font-size: 1.2rem; color: var(--color-text-primary);",
                                    "{v.name}"
                                }
                                p { style: "margin: 0.25rem 0 0; font-size: var(--text-xs); color: var(--color-text-muted);",
                                    "County: {v.county} • ID: {v.id}"
                                }
                            }

                            div { class: "kh-info-grid",
                                div { class: "kh-info-block",
                                    span { class: "kh-info-label", "Applicant / Owner" }
                                    span { class: "kh-info-value", "{v.owner}" }
                                }
                                div { class: "kh-info-block",
                                    span { class: "kh-info-label", "Category" }
                                    span { class: "kh-info-value", "{v.category}" }
                                }
                                div { class: "kh-info-block",
                                    span { class: "kh-info-label", "Email Address" }
                                    span { class: "kh-info-value", "{v.email}" }
                                }
                                div { class: "kh-info-block",
                                    span { class: "kh-info-label", "Phone Number" }
                                    span { class: "kh-info-value", "{v.phone}" }
                                }
                                div { class: "kh-info-block",
                                    span { class: "kh-info-label", "Credentials Document" }
                                    a {
                                        href: "#",
                                        style: "color: var(--color-brand); font-size: var(--text-sm); font-weight: bold; text-decoration: none;",
                                        onclick: move |e| e.prevent_default(),
                                        "📄 Business_License.pdf"
                                    }
                                }
                                div { class: "kh-info-block",
                                    span { class: "kh-info-label", "Requested Action" }
                                    span { class: "kh-info-value", "Stripe Connect Onboarding" }
                                }
                            }

                            div { style: "margin-bottom: 1.75rem;",
                                span { class: "kh-info-label", "Merchant Pitch" }
                                p { style: "margin: 0.35rem 0 0; font-size: var(--text-sm); line-height: 1.5; color: var(--color-text-secondary);",
                                    "{v.description}"
                                }
                            }

                            div { class: "kh-action-row",
                                crate::components::ui::Button {
                                    variant: "outline",
                                    onclick: move |_| {
                                        show_reject_confirm.set(true);
                                    },
                                    "Reject Application"
                                }
                                crate::components::ui::Button {
                                    onclick: move |_| {
                                        on_approve.call(v.id.clone());
                                        inspect_vendor.set(None);
                                    },
                                    "Approve & Activate"
                                }
                            }
                        } else {
                            div { style: "margin-bottom: 1.5rem;",
                                p { style: "margin: 0; font-size: var(--text-sm); color: var(--color-text-secondary);",
                                    "Please provide a feedback reason for rejecting the application for "
                                    strong { "{v.name}" }
                                    ". This rejection reason will be emailed to "
                                    code { "{v.email}" }
                                    "."
                                }
                            }

                            div { style: "margin-bottom: 1.75rem; display: flex; flex-direction: column; gap: 0.5rem;",
                                label { class: "kh-info-label", "Rejection Comments" }
                                textarea {
                                    class: "kh-admin-input",
                                    style: "width: 100%; min-height: 100px; resize: vertical; padding: 0.5rem; font-family: inherit;",
                                    placeholder: "E.g., Business license has expired, or description missing detailed contact information.",
                                    value: "{rejection_reason}",
                                    oninput: move |e| rejection_reason.set(e.value()),
                                }
                            }

                            div { class: "kh-action-row",
                                crate::components::ui::Button {
                                    variant: "outline",
                                    onclick: move |_| show_reject_confirm.set(false),
                                    "Back to Review"
                                }
                                button {
                                    class: "kh-btn-outline",
                                    style: "padding: 0.6rem 1.2rem; font-size: var(--text-sm); border-color: rgb(239, 68, 68); color: rgb(239, 68, 68); font-weight: bold;",
                                    disabled: rejection_reason().trim().is_empty(),
                                    onclick: move |_| {
                                        on_reject.call((v.id.clone(), rejection_reason().clone()));
                                        inspect_vendor.set(None);
                                        show_reject_confirm.set(false);
                                    },
                                    "Confirm Rejection"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
