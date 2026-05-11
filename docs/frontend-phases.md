# KoloHub web shell — phased plan

Dioxus app: `frontend/kolohub-web`. Design tokens: `design-system/tokens.css` + crate `kolohub-theme`.

## Phase 1 — Toolchain + workspace (done)

- Workspace member `kolohub-web` with `dioxus` (features `web`) and path dependency on `kolohub-theme`.
- Minimal `App` and `dioxus::launch(App)`.
- `cargo build -p kolohub-web` checks the native/host build; for the browser bundle you need the WASM target (`rustup target add wasm32-unknown-unknown`) and [Dioxus CLI](https://dioxuslabs.com/learn/0.6/getting_started) (`dx`). From `frontend/kolohub-web`: `dx serve` (flags vary by CLI version).

## Phase 2 — Global styles (done)

- `tokens.css` is bundled via `include_str!(…/design-system/tokens.css)` and `document::Style` (injected into `<head>`).
- Root content mounts under `class: "kh-theme"` so variables and helpers (e.g. `kh-text-gradient`) apply. Dark class toggle comes in Phase 4.

## Phase 3 — Shell layout (done)

- Top glass nav + main content region using `kolohub_theme::dioxus` helpers (`app_shell_style`, `glass_nav_style`) or equivalent classes from `tokens.css`.
- Responsive breakpoints aligned with the design mockups (mobile-first).
- Added reusable Dioxus components (`TopNav`, `HeroSection`) in `frontend/kolohub-web/src/main.rs`.
- Wired public assets (logo/banner/favicon) under `frontend/kolohub-web/public`.

## Phase 4 — Theme toggle (done)

- Added `use_signal(ColorScheme::Light)` in `frontend/kolohub-web/src/main.rs`.
- Toggle button now flips light/dark at runtime by switching root class between `kh-theme` and `kh-theme dark`.
- Theme persistence is implemented in Phase 4.1.

## Phase 4.1 — Theme persistence (done)

- App now reads initial theme from `localStorage` key `kolohub.theme` (web target).
- On theme change, app writes `light`/`dark` back to `localStorage`.
- Non-web targets gracefully default to `ColorScheme::Light`.

## Phase 5 — Hero + CTAs

- First real marketing/home slice: gradient headline, subtitle, purple + emerald buttons, matching `design-system/preview.html` and product UI direction.
- Refactored to align with `Design-UI.png`: Frontend Preview section + integrated design image + vendor/admin highlight cards.
- Expanded navbar to include key customer flow links from `Design-UI.png` (Browse, Details, Cart, Checkout, Track) plus Customer/Vendor entry actions.
- Phase 5.2: responsive layout polish with desktop split preview grid and stronger KPI hierarchy in vendor/admin panels.
- Phase 5.3: mobile navbar simplification (collapsed flow group) and dedicated five-stage customer journey strip.
- Refactor: split `frontend/kolohub-web/src/main.rs` into component modules under `src/components/` and shared styles in `src/styles.rs`.

After Phase 5, start routing (e.g. customer vs vendor shells) or shared components as separate tracks.
