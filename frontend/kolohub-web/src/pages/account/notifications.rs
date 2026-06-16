use dioxus::prelude::*;

#[component]
pub fn AccountNotifications() -> Element {
    rsx! {
        div { class: "kh-animate-fade-in-up",
            div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 2rem;",
                div {
                    h2 { style: "font-size: var(--text-xl); font-weight: 800; margin-bottom: 0.25rem;",
                        "Notification Center"
                    }
                    p { style: "color: var(--color-text-secondary); font-size: var(--text-sm);",
                        "Manage your alerts and marketplace activity."
                    }
                }
                div { style: "display: flex; gap: 0.75rem;",
                    button {
                        class: "kh-btn-outline",
                        style: "font-size: var(--text-xs); height: 2rem; padding: 0 0.75rem;",
                        "Clear All"
                    }
                    button {
                        class: "kh-btn-brand",
                        style: "font-size: var(--text-xs); height: 2rem; padding: 0 0.75rem;",
                        "Mark all as read"
                    }
                }
            }

            // Tabs / Filters
            div { style: "display: flex; gap: 1.5rem; border-bottom: 1px solid var(--color-border-subtle); margin-bottom: 2rem;",
                button { class: "kh-tab active", "All Alerts" }
                button { class: "kh-tab", "Orders" }
                button { class: "kh-tab", "Security" }
                button { class: "kh-tab", "Promotions" }
            }

            // Notification Feed
            div { class: "kh-notif-feed",
                NotificationCard {
                    notif_type: "order",
                    icon: "🚚",
                    title: "Order out for delivery",
                    message: "Your order #KH-2940 containing 'Premium Palm Oil' is out for delivery in Monrovia.",
                    time: "2 minutes ago",
                    unread: true,
                }
                NotificationCard {
                    notif_type: "price",
                    icon: "🏷️",
                    title: "Price Drop Alert",
                    message: "An item in your wishlist 'Fresh Organic Cassava' just dropped in price by 15%.",
                    time: "1 hour ago",
                    unread: false,
                }
                NotificationCard {
                    notif_type: "security",
                    icon: "🛡️",
                    title: "Security Update",
                    message: "A new login was detected from a Chrome browser on a Linux device in Monrovia, LR.",
                    time: "Yesterday at 4:32 PM",
                    unread: false,
                }
                NotificationCard {
                    notif_type: "order",
                    icon: "✅",
                    title: "Order Delivered",
                    message: "Order #KH-2891 has been successfully delivered and signed for.",
                    time: "2 days ago",
                    unread: false,
                }
            }
        }
    }
}

#[component]
fn NotificationCard(
    notif_type: String,
    icon: String,
    title: String,
    message: String,
    time: String,
    unread: bool,
) -> Element {
    let unread_class = if unread { "unread" } else { "" };

    rsx! {
        div { class: "kh-notif-card {unread_class}",
            div { class: "kh-notif-card-icon {notif_type}", "{icon}" }
            div { class: "kh-notif-card-body",
                div { style: "display: flex; justify-content: space-between; align-items: flex-start;",
                    h4 { class: "kh-notif-card-title", "{title}" }
                    span { class: "kh-notif-card-time", "{time}" }
                }
                p { class: "kh-notif-card-msg", "{message}" }
                if unread {
                    div { style: "margin-top: 0.75rem;",
                        button { class: "kh-notif-action-btn", "Mark as read" }
                    }
                }
            }
        }
    }
}
