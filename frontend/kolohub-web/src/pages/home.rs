use dioxus::prelude::*;

use crate::components::{
    dashboard::DashboardHighlights, flow_strip::CustomerFlowStrip, preview::MarketplacePreview,
};

/// Landing page — the main marketing / preview screen.
#[component]
pub fn Home() -> Element {
    rsx! {
        MarketplacePreview {}
        CustomerFlowStrip {}
        DashboardHighlights {}
    }
}
