//! Radius, shadow, blur — CSS-ready strings.

pub struct Radius;

impl Radius {
    pub const SM: &'static str = "0.375rem";
    pub const MD: &'static str = "0.5rem";
    pub const LG: &'static str = "0.75rem";
    pub const XL: &'static str = "1rem";
    pub const FULL: &'static str = "9999px";
}

pub struct Shadow;

impl Shadow {
    pub const NAV_LIGHT: &'static str =
        "0 10px 15px -3px rgb(0 0 0 / 0.08), 0 4px 6px -4px rgb(0 0 0 / 0.06)";
    pub const NAV_DARK: &'static str =
        "0 10px 15px -3px rgb(0 0 0 / 0.4), 0 4px 6px -4px rgb(0 0 0 / 0.35)";
    pub const CARD_LIGHT: &'static str =
        "0 1px 3px 0 rgb(0 0 0 / 0.06), 0 1px 2px -1px rgb(0 0 0 / 0.06)";
    pub const CARD_DARK: &'static str =
        "0 1px 3px 0 rgb(0 0 0 / 0.35), 0 1px 2px -1px rgb(0 0 0 / 0.3)";
}

pub struct Blur;

impl Blur {
    /// `--blur-glass`
    pub const GLASS_PX: &'static str = "12px";
}
