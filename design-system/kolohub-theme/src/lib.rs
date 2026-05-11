//! KoloHub design tokens mirrored from `design-system/tokens.css`.
//!
//! Use with Dioxus via inline `style`, `class` (with linked CSS variables), or by
//! formatting CSS strings:
//!
//! ```ignore
//! use kolohub_theme::{palette, semantic::ColorScheme};
//!
//! let scheme = ColorScheme::Light;
//! let bg = format!(
//!     "linear-gradient(to bottom right, {}, {})",
//!     scheme.page_gradient_from(),
//!     scheme.page_gradient_to(),
//! );
//! ```

#![forbid(unsafe_code)]

pub mod dioxus;
pub mod hsl;
pub mod layout;
pub mod palette;
pub mod semantic;
pub mod typography;

pub use hsl::{css_hsl, ShadcnSlots};
pub use layout::Shadow;
pub use semantic::ColorScheme;
pub use typography::{FontFamily, FontSize, FontWeight, LineHeight};
