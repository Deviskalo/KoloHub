//! Inline `style` helpers for Dioxus RSX (`style: "{app_shell_style(scheme)}"`).
//!
//! Prefer loading `design-system/tokens.css` and toggling `class="dark"` on the root
//! for full parity with CSS utilities; use these when you need programmatic colors.

use crate::typography::FontFamily;
use crate::{layout::Blur, ColorScheme};

/// Root layout: gradient background, text color, sans font stack.
#[must_use]
pub fn app_shell_style(scheme: ColorScheme) -> String {
    format!(
        "min-height: 100vh; background: {}; color: {}; font-family: {};",
        scheme.page_background_css(),
        scheme.text_primary(),
        FontFamily::SANS
    )
}

/// Floating nav bar — glass, border, shadow (Iskalo-style).
#[must_use]
pub fn glass_nav_style(scheme: ColorScheme) -> String {
    format!(
        "background: {}; -webkit-backdrop-filter: blur({}); backdrop-filter: blur({}); border: 1px solid {}; box-shadow: {};",
        scheme.surface_glass(),
        Blur::GLASS_PX,
        Blur::GLASS_PX,
        scheme.border_subtle(),
        scheme.shadow_nav()
    )
}

#[must_use]
pub fn solid_brand_button_style(scheme: ColorScheme) -> String {
    format!(
        "display: inline-flex; align-items: center; justify-content: center; padding: 0.625rem 2rem; font-size: {}; font-weight: {}; color: {}; background-color: {}; border: none; border-radius: {}; cursor: pointer;",
        crate::typography::FontSize::SM,
        crate::typography::FontWeight::MEDIUM,
        scheme.text_on_brand(),
        scheme.brand(),
        crate::layout::Radius::MD
    )
}

#[must_use]
pub fn solid_commerce_button_style(scheme: ColorScheme) -> String {
    format!(
        "display: inline-flex; align-items: center; justify-content: center; padding: 0.625rem 2rem; font-size: {}; font-weight: {}; color: #fff; background-color: {}; border: none; border-radius: {}; cursor: pointer;",
        crate::typography::FontSize::SM,
        crate::typography::FontWeight::MEDIUM,
        scheme.accent(),
        crate::layout::Radius::MD
    )
}
