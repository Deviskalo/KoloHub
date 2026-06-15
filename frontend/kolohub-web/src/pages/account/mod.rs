pub mod addresses;
pub mod help_center;
pub mod order_details;
pub mod orders;
pub mod payments;
pub mod profile;
pub mod reviews;
pub mod security;
pub mod wishlist;

use crate::router::Route;
use dioxus::prelude::*;

#[component]
pub fn AccountLayout() -> Element {
    rsx! {
        main { class: "kh-main-shell",
            div { class: "kh-animate-fade-in-up",
                h1 { style: "font-size: 2rem; font-weight: var(--font-weight-bold); margin-bottom: 2rem;", "My Account" }

                div { class: "kh-account-layout",
                    aside { class: "kh-account-nav",
                        Link {
                            to: Route::AccountProfile {},
                            class: "kh-account-link",
                            active_class: "active",
                            span { "👤" }
                            "Profile Info"
                        }
                        Link {
                            to: Route::AccountOrders {},
                            class: "kh-account-link",
                            active_class: "active",
                            span { "🛍️" }
                            "Order History"
                        }
                        Link {
                            to: Route::AccountAddresses {},
                            class: "kh-account-link",
                            active_class: "active",
                            span { "📍" }
                            "Saved Addresses"
                        }
                        Link {
                            to: Route::Track {},
                            class: "kh-account-link",
                            span { "🚚" }
                            "Track Orders"
                        }
                        Link {
                            to: Route::AccountPayments {},
                            class: "kh-account-link",
                            active_class: "active",
                            span { "💳" }
                            "Payments"
                        }
                        Link {
                            to: Route::AccountSecurity {},
                            class: "kh-account-link",
                            active_class: "active",
                            span { "🔒" }
                            "Security"
                        }
                        Link {
                            to: Route::AccountWishlist {},
                            class: "kh-account-link",
                            active_class: "active",
                            span { "❤️" }
                            "Wishlist"
                        }
                        Link {
                            to: Route::AccountReviews {},
                            class: "kh-account-link",
                            active_class: "active",
                            span { "★" }
                            "My Reviews"
                        }
                        Link {
                            to: Route::AccountHelp {},
                            class: "kh-account-link",
                            active_class: "active",
                            span { "❓" }
                            "Help Center"
                        }
                        div { style: "margin-top: 2rem; padding-top: 1rem; border-top: 1px solid var(--color-border-subtle);",
                            button {
                                class: "kh-account-link",
                                style: "width: 100%; background: none; border: none; cursor: pointer; color: #ef4444;",
                                span { "🚪" }
                                "Sign Out"
                            }
                        }
                    }

                    div { class: "kh-account-content",
                        Outlet::<Route> {}
                    }
                }
            }
        }
    }
}
