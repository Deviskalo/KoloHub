use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "kh-footer",
            div { class: "kh-footer-top",
                div { class: "kh-footer-brand",
                    h2 { "KoloHub" }
                    p { "Liberia's premier local marketplace. Shop the best of Monvoria and beyond, instantly." }
                }
                div { class: "kh-footer-links",
                    h3 { "Shop" }
                    a { href: "/browse", "All Products" }
                    a { href: "/browse?query=electronics", "Electronics" }
                    a { href: "/browse?query=fashion", "Fashion" }
                    a { href: "/browse?query=food", "Groceries" }
                }
                div { class: "kh-footer-links",
                    h3 { "Sell" }
                    a { href: "/dashboard", "Vendor Dashboard" }
                    a { href: "#", "Seller Guidelines" }
                    a { href: "#", "Payouts & Logistics" }
                }
                div { class: "kh-footer-links",
                    h3 { "Support" }
                    a { href: "#", "Help Center" }
                    a { href: "/track", "Track Order" }
                    a { href: "#", "Contact Us" }
                    a { href: "#", "Returns" }
                }
            }
            div { class: "kh-footer-bottom",
                p { "© 2026 KoloHub LLC. All rights reserved." }
                div { class: "kh-footer-legal",
                    a { href: "#", "Terms of Service" }
                    a { href: "#", "Privacy Policy" }
                }
            }
        }
    }
}
