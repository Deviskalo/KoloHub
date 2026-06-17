use dioxus::prelude::*;

struct CategoryCard {
    icon: &'static str,
    name: &'static str,
    description: &'static str,
    count: &'static str,
    slug: &'static str,
    color: &'static str,
}

const CATEGORIES: &[CategoryCard] = &[
    CategoryCard {
        icon: "🥬",
        name: "Fresh Produce",
        description: "Farm-fresh fruits, vegetables, and organic goods",
        count: "1,240+ items",
        slug: "produce",
        color: "rgba(34,197,94,0.12)",
    },
    CategoryCard {
        icon: "📱",
        name: "Electronics",
        description: "Phones, accessories, gadgets, and tech supplies",
        count: "870+ items",
        slug: "electronics",
        color: "rgba(99,102,241,0.12)",
    },
    CategoryCard {
        icon: "👕",
        name: "Fashion & Apparel",
        description: "Clothing, footwear, bags, and accessories",
        count: "3,100+ items",
        slug: "fashion",
        color: "rgba(236,72,153,0.12)",
    },
    CategoryCard {
        icon: "🏠",
        name: "Home & Living",
        description: "Furniture, décor, kitchenware, and appliances",
        count: "2,050+ items",
        slug: "home",
        color: "rgba(245,158,11,0.12)",
    },
    CategoryCard {
        icon: "💊",
        name: "Health & Wellness",
        description: "Supplements, personal care, and medical supplies",
        count: "640+ items",
        slug: "health",
        color: "rgba(239,68,68,0.12)",
    },
    CategoryCard {
        icon: "📚",
        name: "Books & Education",
        description: "Textbooks, novels, stationery, and learning tools",
        count: "510+ items",
        slug: "books",
        color: "rgba(14,165,233,0.12)",
    },
    CategoryCard {
        icon: "🍳",
        name: "Food & Beverages",
        description: "Packaged foods, snacks, drinks, and local delicacies",
        count: "1,800+ items",
        slug: "food",
        color: "rgba(249,115,22,0.12)",
    },
    CategoryCard {
        icon: "🌱",
        name: "Agriculture & Farming",
        description: "Seeds, tools, fertilizers, and livestock supplies",
        count: "720+ items",
        slug: "agriculture",
        color: "rgba(22,163,74,0.12)",
    },
    CategoryCard {
        icon: "⚽",
        name: "Sports & Outdoors",
        description: "Equipment, activewear, and adventure gear",
        count: "430+ items",
        slug: "sports",
        color: "rgba(168,85,247,0.12)",
    },
    CategoryCard {
        icon: "🧸",
        name: "Toys & Kids",
        description: "Toys, baby products, and children's supplies",
        count: "380+ items",
        slug: "toys",
        color: "rgba(251,191,36,0.12)",
    },
    CategoryCard {
        icon: "🔧",
        name: "Tools & Hardware",
        description: "Power tools, building materials, and DIY supplies",
        count: "290+ items",
        slug: "tools",
        color: "rgba(115,115,115,0.12)",
    },
    CategoryCard {
        icon: "💄",
        name: "Beauty & Cosmetics",
        description: "Skincare, makeup, hair care, and fragrances",
        count: "960+ items",
        slug: "beauty",
        color: "rgba(244,63,94,0.12)",
    },
];

#[component]
pub fn Categories() -> Element {
    rsx! {
        main { class: "kh-main-shell kh-animate-fade-in-up",

            // Hero Header
            div { class: "kh-categories-hero",
                div { class: "kh-categories-hero-content",
                    span { class: "kh-categories-hero-badge", "🛍️ Explore KoloHub" }
                    h1 { class: "kh-categories-hero-title", "Find What You Need" }
                    p { class: "kh-categories-hero-subtitle",
                        "Browse over 13,000+ products across every category. From fresh produce to the latest electronics — it's all here."
                    }
                }
            }

            // Categories Grid
            section { class: "kh-categories-section",
                div { class: "kh-section-header",
                    h2 { class: "kh-section-title", "All Categories" }
                    p { class: "kh-section-subtitle", "Click a category to explore its full catalog" }
                }

                div { class: "kh-categories-grid",
                    for cat in CATEGORIES {
                        a {
                            href: "/browse/{cat.slug}",
                            class: "kh-category-card",
                            style: "--cat-color: {cat.color};",

                            div { class: "kh-category-card-icon",
                                span { "{cat.icon}" }
                            }
                            div { class: "kh-category-card-body",
                                h3 { class: "kh-category-card-name", "{cat.name}" }
                                p { class: "kh-category-card-desc", "{cat.description}" }
                                span { class: "kh-category-card-count", "{cat.count}" }
                            }
                            span { class: "kh-category-card-arrow", "→" }
                        }
                    }
                }
            }

            // Popular / Featured CTA strip
            div { class: "kh-categories-cta",
                div { class: "kh-categories-cta-inner",
                    span { style: "font-size: 2rem;", "✨" }
                    div {
                        h3 { "Can't find what you're looking for?" }
                        p { "Search across all categories or contact a KoloHub vendor directly." }
                    }
                    a { href: "/browse", class: "kh-btn-brand",
                        "Browse All Products →"
                    }
                }
            }
        }
    }
}
