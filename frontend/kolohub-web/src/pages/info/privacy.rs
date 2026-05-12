use dioxus::prelude::*;

#[component]
pub fn Privacy() -> Element {
    rsx! {
        div { class: "kh-container kh-section",
            style: "max-width: 900px; margin: 0 auto; padding-top: 8rem; padding-bottom: 6rem;",
            
            div { class: "kh-text-center", style: "margin-bottom: 4rem;",
                h1 { class: "kh-text-gradient", style: "font-size: 3.5rem; margin-bottom: 1rem;", "Privacy Policy" }
                p { style: "color: var(--color-text-secondary); font-size: 1.1rem;", "Your data is yours. Here is how we protect it." }
            }

            div { class: "kh-card kh-surface-glass", style: "padding: 3rem;",
                div { class: "kh-prose",
                    h2 { "1. Information We Collect" }
                    p { "To provide our services, we collect three types of information:" }
                    ul {
                        li { b { "Provided by You:" } " Name, phone number, delivery address, and payment preferences." }
                        li { b { "Automatically Collected:" } " IP address, browser type, and device identifiers used to access our platform." }
                        li { b { "Transaction Data:" } " History of purchases, vendor interactions, and delivery statuses." }
                    }

                    h2 { "2. How We Use Your Information" }
                    p { "We use your data strictly for operational purposes:" }
                    ul {
                        li { "Processing and delivering your orders." }
                        li { "Sending order updates and security alerts." }
                        li { "Verifying vendor identities to prevent fraud." }
                        li { "Improving the platform through anonymous analytics." }
                    }

                    h2 { "3. Data Sharing" }
                    p { "KoloHub does not sell or lease your personal information to marketers. We only share data in these cases:" }
                    ul {
                        li { b { "With Riders/Logistics:" } " We provide your name, phone, and address to ensure delivery." }
                        li { b { "With Vendors:" } " We share necessary details to fulfill your order." }
                        li { b { "Legal Requirements:" } " When required by Liberian law enforcement or court orders." }
                    }

                    h2 { "4. Your Rights" }
                    p { "You have full control over your data on KoloHub. You can:" }
                    ul {
                        li { "Access and download your account data." }
                        li { "Request correction of inaccurate information." }
                        li { "Request deletion of your account and associated personal data (subject to legal retention requirements for tax/audit)." }
                    }

                    h2 { "5. Cookies" }
                    p { "We use 'cookies' to keep you logged in and remember your preferences. You can disable cookies in your browser settings, but some features of KoloHub may not work correctly." }

                    h2 { "6. Security" }
                    p { "We implement technical and organizational measures including encryption and firewalls to protect your data. However, no internet-based service is 100% secure, and we encourage you to use strong, unique passwords." }

                    div { style: "margin-top: 3rem; padding: 2rem; border-radius: var(--radius-lg); background: var(--color-surface-muted);",
                        p { style: "margin: 0; font-size: 0.9rem;", "For privacy inquiries, contact privacy@kolohub.com" }
                    }
                }
            }
        }
    }
}
