//! Font stacks and type scale — CSS-ready strings.

/// Sans stack (matches `tokens.css` `--font-sans`).
pub struct FontFamily;

impl FontFamily {
    pub const SANS: &'static str = r#"ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji"#;

    pub const MONO: &'static str = r#"ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace"#;
}

/// Rem sizes (`--text-*`).
pub struct FontSize;

impl FontSize {
    pub const XS: &'static str = "0.75rem";
    pub const SM: &'static str = "0.875rem";
    pub const BASE: &'static str = "1rem";
    pub const LG: &'static str = "1.125rem";
    pub const XL: &'static str = "1.25rem";
    pub const XXL: &'static str = "1.5rem";
    pub const XXXL: &'static str = "1.875rem";
    pub const XXXXL: &'static str = "2.25rem";
    pub const HERO_SM: &'static str = "3rem";
    pub const HERO_MD: &'static str = "3.75rem";
    pub const HERO_LG: &'static str = "4.5rem";
}

pub struct FontWeight;

impl FontWeight {
    pub const NORMAL: u16 = 400;
    pub const MEDIUM: u16 = 500;
    pub const SEMIBOLD: u16 = 600;
    pub const BOLD: u16 = 700;
}

pub struct LineHeight;

impl LineHeight {
    pub const TIGHT: f32 = 1.25;
    pub const SNUG: f32 = 1.375;
    pub const NORMAL: f32 = 1.5;
    pub const RELAXED: f32 = 1.625;
}
