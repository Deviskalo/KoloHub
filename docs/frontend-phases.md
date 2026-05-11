# KoloHub web shell — phased plan

Dioxus app: `frontend/kolohub-web`. Design tokens: `design-system/tokens.css` + crate `kolohub-theme`.

## Phase 1 — Toolchain + workspace (done)

- Workspace member `kolohub-web` with `dioxus` (features `web`) and path dependency on `kolohub-theme`.
- Minimal `App` and `dioxus::launch(App)`.
- `cargo build -p kolohub-web` checks the native/host build; for the browser bundle you need the WASM target (`rustup target add wasm32-unknown-unknown`) and [Dioxus CLI](https://dioxuslabs.com/learn/0.6/getting_started) (`dx`). From `frontend/kolohub-web`: `dx serve` (flags vary by CLI version).

## Phase 2 — Global styles (done)

- `tokens.css` is bundled via `include_str!(…/design-system/tokens.css)` and `document::Style` (injected into `<head>`).
- Root content mounts under `class: "kh-theme"` so variables and helpers (e.g. `kh-text-gradient`) apply. Dark class toggle comes in Phase 4.

## Phase 3 — Shell layout

- Top glass nav + main content region using `kolohub_theme::dioxus` helpers (`app_shell_style`, `glass_nav_style`) or equivalent classes from `tokens.css`.
- Responsive breakpoints aligned with the design mockups (mobile-first).

## Phase 4 — Theme toggle

- `use_signal` for `ColorScheme`; toggle `dark` on `<html>` or root (via `dioxus::document` or a root wrapper class).
- Optional: persist choice in `localStorage` (small `use_effect` + eval or web-sys).

## Phase 5 — Hero + CTAs

- First real marketing/home slice: gradient headline, subtitle, purple + emerald buttons, matching `design-system/preview.html` and product UI direction.

After Phase 5, start routing (e.g. customer vs vendor shells) or shared components as separate tracks.
