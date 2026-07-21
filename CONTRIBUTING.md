# Contributing Guidelines

Thank you for contributing to KoloHub! This repository houses the unified frontend web and mobile codebase alongside components representing local African e-commerce features.

---

## 🛠 Developer Setup

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable edition)
- Dioxus CLI:
  ```bash
  cargo install dioxus-cli --tag v0.6.0
  ```

### Startup Development Server

Execute the start script to launch the local Dioxus frontend hot-reloader:

```bash
./startDev.sh
```

---

## 🎨 UI & Styling Architecture

To maintain visual parity across all pages and views, we use a custom design token system located in `design-system/tokens.css` referenced by `src/styles.rs`.

### Coding Standards

1. **Class Naming**: Always prefix layout class classes with the `kh-` namespace (e.g. `.kh-dash-card`, `.kh-badge`).
2. **Css Tokens**: Never use hardcoded hex/rgb codes. Prefer CSS color variables:
   - `--color-brand` (Primary Brand Blue/Purple)
   - `--color-brand-muted` (Brand Accent highlight)
   - `--color-surface` / `--color-surface-elevated` (Base layouts)
   - `--color-border-subtle` / `--color-border-strong` (Borders)
   - `--radius-lg` / `--radius-xl` / `--radius-full` (Border radius tokens)
3. **Responsive**: Use `.kh-admin-dash-grid` and inline flex wrappers to guarantee responsive layouts on both Mobile and Wide Desktop viewpoints.

---

## ⚙ Rust & Dioxus Best Practices

1. **State Management**:
   - Utilize Dioxus `use_signal(|| init)` for component-isolated states.
   - Utilize shared `Signal<T>` when propagating data changes across dashboard subcomponents.
2. **Dioxus 0.6 rsx! Constraints**:
   - **No Match Blocks in rsx!**: Precompute component structures or use simple `if/else` checks. Dioxus 0.6 rsx parser will throw compiler errors on match branches.
   - **Binding Format Interpolations**: To avoid format parser glitches, pre-format values using `format!()` calls outside the `rsx!` block and insert the plain string reference.
   - **Cloning in closures**: Explicitly clone signals or variables before moving them into event handler callbacks (e.g. `onclick: move |_| { ... }`).

---

## 📥 Development Submission Process

1. **Create Branch**: Create a feature branch off of `main`:
   ```bash
   git checkout -b feature/your-awesome-change
   ```
2. **Code Verification**: Run code compilation audits before submitting:
   ```bash
   cargo check
   cargo fmt --check
   ```
3. **Commit Messages**: Follow standard descriptive commit structure:
   - `feat: Add SVG timelines to Admin Command Center`
   - `fix: Correct match interpolation error in rsx!`
4. **Submit PR**: Open a pull request against the `main` branch. Complete all sections of the `pull_request_template.md`.
