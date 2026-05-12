use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "kh-footer",
            div { class: "kh-footer-top",
                div { class: "kh-footer-brand",
                    h2 { "KoloHub" }
                    p {
                        "Liberia's premier local marketplace. Shop the best of Monvoria and beyond, instantly."
                    }
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
                    a { href: "/seller-guidelines", "Seller Guidelines" }
                    a { href: "/payouts-logistics", "Payouts & Logistics" }
                }
                div { class: "kh-footer-links",
                    h3 { "Support" }
                    a { href: "/help", "Help Center" }
                    a { href: "/track", "Track Order" }
                    a { href: "/contact", "Contact Us" }
                    a { href: "/returns", "Returns" }
                }
            }
            div { class: "kh-footer-bottom",
                p { "© 2026 KoloHub. All rights reserved." }
                div { class: "kh-footer-legal",
                    a { href: "/terms", "Terms of Service" }
                    a { href: "/privacy", "Privacy Policy" }
                }
            }
        }
    }
}
