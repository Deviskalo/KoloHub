use dioxus::prelude::*;

#[component]
pub fn Terms() -> Element {
    rsx! {
        div { class: "kh-container kh-section",
            style: "max-width: 900px; margin: 0 auto; padding-top: 8rem; padding-bottom: 6rem;",
            
            div { class: "kh-text-center", style: "margin-bottom: 4rem;",
                h1 { class: "kh-text-gradient", style: "font-size: 3.5rem; margin-bottom: 1rem;", "Terms of Service" }
                p { style: "color: var(--color-text-secondary); font-size: 1.1rem;", "Last updated: May 12, 2026" }
            }

            div { class: "kh-card kh-surface-glass", style: "padding: 3rem;",
                div { class: "kh-prose",
                    h2 { "1. Acceptance of Terms" }
                    p { "KoloHub is owned and operated by KoloHub LLC. By accessing our platform, website, or mobile application, you agree to comply with and be bound by these Terms of Service. If you do not agree, please do not use the platform." }

                    h2 { "2. Eligibility" }
                    p { "You must be at least 18 years old and have the legal capacity to enter into binding contracts under Liberian law. If you are using KoloHub on behalf of a business, you represent that you have the authority to bind that business to these terms." }

                    h2 { "3. User Responsibilities" }
                    ul {
                        li { b { "Account Security:" } " You are responsible for all activity on your account. You must notify us immediately of any unauthorized use." }
                        li { b { "Accuracy:" } " You must provide true, accurate, and complete information during registration and keep it updated." }
                    }

                    h2 { "4. Buying on KoloHub" }
                    p { "When you buy an item, you are entering into a direct contract with the Vendor. KoloHub facilitates the transaction but is not a party to the sale unless otherwise stated." }
                    ul {
                        li { "Prices are listed in USD/LRD." }
                        li { "Orders are subject to acceptance by the vendor." }
                    }

                    h2 { "5. Selling on KoloHub" }
                    p { "Vendors must follow the 'Seller Guidelines' and are responsible for the quality, safety, and legality of the items they list." }

                    h2 { "6. Intellectual Property" }
                    p { "All content on KoloHub (logos, text, graphics, software) is the property of KoloHub or its content suppliers and is protected by intellectual property laws. You may not reproduce or distribute any part of the platform without our written consent." }

                    h2 { "7. Termination" }
                    p { "We reserve the right to suspend or terminate your account at our sole discretion, without notice, for conduct that we believe violates these terms or is harmful to other users or our business interests." }

                    h2 { "8. Dispute Resolution" }
                    p { "Any disputes arising from these terms or your use of the platform shall be governed by the laws of the Republic of Liberia and shall be subject to the exclusive jurisdiction of the courts in Monrovia." }

                    div { style: "margin-top: 3rem; padding: 2rem; border-radius: var(--radius-lg); background: var(--color-surface-muted);",
                        p { style: "margin: 0; font-size: 0.9rem;", "Questions about these terms should be sent to legal@kolohub.com" }
                    }
                }
            }
        }
    }
}
