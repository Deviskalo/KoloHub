# Changelog

All notable changes to the KoloHub platform will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.3.0] - 2026-07-21

### Added

- **Admin Command Center Tab System**: Integrated dynamic three-tab view within `AdminOverview` (Overview, Vendor Directory, Approval Queue) running reactive state signals.
- **High-Fidelity SVG Charts**:
  - Main area-under-line transaction history timeline with custom marker dots, grid lines, and interactive toggles for Revenue & User timeline datagroups.
  - Monthly registered users bar metrics styling with rich custom CSS gradients and responsive vertical heights.
- **Recent Platform Activity Feed**: An 8-item timeline logger indicating system actions, security warnings, signups, and high-value orders with colored status badge pills.
- **Sparkline KPI cards**: Real-time sales metrics including Total Users, Active Vendor count, Total Revenue, Conversion Rate, and AOV, complete with inline micro-SVG trendlines.
- **Admin Sidebar Routing**: Integrated supervisory modules inside the primary `DashboardLayout` container.

### Changed

- Refactored matching syntax logic in `admin_overview.rs` into pre-computed variable lookups to prevent Dioxus 0.6 compiler limitations.

---

## [0.2.0] - 2026-06-15

### Added

- **Buyer Account Portal**: Built extensive user modules including Profile Settings, Address Book, Order Actions log, Password Security, Payment Wallets, Wishlist grids, and Help channels.
- **Global Categories Hub**: Built a `/categories` layout utilizing a 12-column grid system with animated hover effects and detailed subcategory indicators.
- **Notification Center topbar dropdown**: Interactive navigation icon with badge counts and scrollable glassmorphic notifications.
- **Flash Deals Timer**: Added ticking cross-platform WASM responsive counter to display remaining flash-sale limits.
- **Aesthetic Refinement Pack**: Added dark-mode borders resolver, custom global scrollbar style templates, hero floating emojis, and gradient background orbs.

### Changed

- Shifted root `/track` endpoints to nested `/account/track` structures for better routing alignment.

---

## [0.1.0] - 2026-05-10

### Added

- **Vendor Space**: Created responsive product add/edit dialogs, order fulfillment lists, and sales tracking grids.
- **Global Storefront**: Added search bar elements, dynamic product catalog previews, and marketplace category sliders.
- **Core Architecture Core**: Initial routing structures and layouts in Dioxus.
