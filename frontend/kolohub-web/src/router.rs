//! Route definitions + AppLayout — the persistent shell wrapper for all pages.

use dioxus::prelude::*;
use kolohub_theme::ColorScheme;

use crate::components::top_bar::TopBar;
use crate::pages::{
    account::addresses::AccountAddresses,
    account::order_details::AccountOrderDetails,
    account::orders::AccountOrders,
    account::payments::AccountPayments,
    account::profile::AccountProfile,
    account::security::AccountSecurity,
    account::AccountLayout,
    browse::Browse,
    cart::Cart,
    category::CategoryPage,
    checkout::Checkout,
    dashboard::admin_overview::AdminOverview,
    dashboard::orders::DashboardOrders,
    dashboard::product_form::{DashboardProductAdd, DashboardProductEdit},
    dashboard::products::DashboardProducts,
    dashboard::vendor_overview::VendorOverview,
    dashboard::DashboardLayout,
    home::Home,
    info::contact_us::ContactUs,
    info::help_center::HelpCenter,
    info::payouts_logistics::PayoutsLogistics,
    info::privacy::Privacy,
    info::returns::Returns,
    info::seller_guidelines::SellerGuidelines,
    info::terms::Terms,
    not_found::NotFound,
    product::Product,
    track::Track,
};
use crate::theme::{load_theme_from_storage, save_theme_to_storage};

/// Top-level application routes.
#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppLayout)]
        #[layout(StorefrontLayout)]
            #[route("/")]
            Home {},
            #[route("/browse?:query")]
            Browse { query: String },
            #[route("/category/:name")]
            CategoryPage { name: String },
            #[route("/product/:id")]
            Product { id: String },
            #[route("/cart")]
            Cart {},
            #[route("/checkout")]
            Checkout {},
            #[route("/track")]
            Track {},
            #[route("/seller-guidelines")]
            SellerGuidelines {},
            #[route("/payouts-logistics")]
            PayoutsLogistics {},
            #[route("/help")]
            HelpCenter {},
            #[route("/contact")]
            ContactUs {},
            #[route("/returns")]
            Returns {},
            #[route("/terms")]
            Terms {},
            #[route("/privacy")]
            Privacy {},
        #[end_layout]
        
        #[layout(AccountLayout)]
            #[route("/account")]
            AccountProfile {},
            #[route("/account/orders")]
            AccountOrders {},
            #[route("/account/orders/:id")]
            AccountOrderDetails { id: String },
            #[route("/account/addresses")]
            AccountAddresses {},
            #[route("/account/security")]
            AccountSecurity {},
            #[route("/account/payments")]
            AccountPayments {},
        #[end_layout]

        #[layout(DashboardLayout)]
            #[route("/dashboard")]
            VendorOverview {},
            #[route("/dashboard/products")]
            DashboardProducts {},
            #[route("/dashboard/products/add")]
            DashboardProductAdd {},
            #[route("/dashboard/products/edit/:id")]
            DashboardProductEdit { id: String },
            #[route("/dashboard/orders")]
            DashboardOrders {},
            #[route("/dashboard/admin")]
            AdminOverview {},
        #[end_layout]
    #[end_layout]
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

/// Persistent shell: theme signal + TopBar + page outlet.
#[component]
fn AppLayout() -> Element {
    let mut color_scheme = use_signal(load_theme_from_storage);
    let root_class = match color_scheme() {
        ColorScheme::Light => "kh-theme",
        ColorScheme::Dark => "kh-theme dark",
    };

    use_effect(move || {
        save_theme_to_storage(color_scheme());
    });

    rsx! {
        div { class: "{root_class}",
            TopBar {
                is_dark: matches!(color_scheme(), ColorScheme::Dark),
                on_toggle: move |_| {
                    color_scheme.set(color_scheme().toggle());
                },
            }
            Outlet::<Route> {}
        }
    }
}

/// Layout wrapper that appends the global footer.
#[component]
fn StorefrontLayout() -> Element {
    rsx! {
        Outlet::<Route> {}
        crate::components::footer::Footer {}
    }
}
