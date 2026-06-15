pub mod addresses;
pub mod help_center;
pub mod order_details;
pub mod orders;
pub mod payments;
pub mod profile;
pub mod reviews;
pub mod rewards;
pub mod security;
pub mod settings;
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
                        // Identity Header
                        div { class: "kh-sidebar-identity",
                            img {
                                class: "kh-sidebar-avatar",
                                src: "https://images.unsplash.com/photo-1535713875002-d1d0cf377fde?w=100",
                                alt: "Johnathan Doe"
                            }
                            div { class: "kh-sidebar-info",
                                h4 { "Johnathan Doe" }
                                p { "Elite Buyer • Member since 2024" }
                            }
                        }

                        // Group: Shopping
                        div { class: "kh-sidebar-group",
                            label { class: "kh-sidebar-label", "Shopping & Logistics" }
                            Link { to: Route::AccountOrders {}, class: "kh-account-link", active_class: "active",
                                span { "🛍️" }
                                "Order History"
                            }
                            Link { to: Route::AccountAddresses {}, class: "kh-account-link", active_class: "active",
                                span { "📍" }
                                "Saved Addresses"
                            }
                            Link { to: Route::AccountTrack {}, class: "kh-account-link", active_class: "active",
                                span { "🚚" }
                                "Track Orders"
                            }
                            Link { to: Route::AccountWishlist {}, class: "kh-account-link", active_class: "active",
                                span { "❤️" }
                                "My Wishlist"
                            }
                        }

                        // Group: Reputation & Rewards
                        div { class: "kh-sidebar-group",
                            label { class: "kh-sidebar-label", "Reputation & Rewards" }
                            Link { to: Route::AccountReviews {}, class: "kh-account-link", active_class: "active",
                                span { "★" }
                                "My Reviews"
                            }
                            Link { to: Route::AccountRewards {}, class: "kh-account-link", active_class: "active",
                                span { "💎" }
                                "Kolo Rewards"
                            }
                        }

                        // Group: Account & Support
                        div { class: "kh-sidebar-group",
                            label { class: "kh-sidebar-label", "Account & Settings" }
                            Link { to: Route::AccountProfile {}, class: "kh-account-link", active_class: "active",
                                span { "👤" }
                                "Personal Info"
                            }
                            Link { to: Route::AccountPayments {}, class: "kh-account-link", active_class: "active",
                                span { "💳" }
                                "Payments"
                            }
                            Link { to: Route::AccountSecurity {}, class: "kh-account-link", active_class: "active",
                                span { "🔒" }
                                "Security & Access"
                            }
                            Link { to: Route::AccountSettings {}, class: "kh-account-link", active_class: "active",
                                span { "⚙️" }
                                "Notifications"
                            }
                            Link { to: Route::AccountHelp {}, class: "kh-account-link", active_class: "active",
                                span { "❓" }
                                "Help Center"
                            }
                        }

                        div { style: "margin-top: 1rem; padding-top: 1rem; border-top: 1px solid var(--color-border-subtle);",
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
