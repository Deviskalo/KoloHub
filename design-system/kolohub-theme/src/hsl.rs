//! shadcn-style HSL components (space-separated) for `hsl(var(--primary))` patterns.

/// HSL triples without the `hsl()` wrapper — same as `tokens.css` `:root` / `.dark`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShadcnSlots {
    pub background: &'static str,
    pub foreground: &'static str,
    pub card: &'static str,
    pub card_foreground: &'static str,
    pub popover: &'static str,
    pub popover_foreground: &'static str,
    pub primary: &'static str,
    pub primary_foreground: &'static str,
    pub secondary: &'static str,
    pub secondary_foreground: &'static str,
    pub muted: &'static str,
    pub muted_foreground: &'static str,
    pub accent: &'static str,
    pub accent_foreground: &'static str,
    pub destructive: &'static str,
    pub destructive_foreground: &'static str,
    pub border: &'static str,
    pub input: &'static str,
    pub ring: &'static str,
}

impl ShadcnSlots {
    pub const LIGHT: Self = Self {
        background: "0 0% 100%",
        foreground: "0 0% 9%",
        card: "0 0% 100%",
        card_foreground: "0 0% 9%",
        popover: "0 0% 100%",
        popover_foreground: "0 0% 9%",
        primary: "271 81% 56%",
        primary_foreground: "0 0% 98%",
        secondary: "270 100% 98%",
        secondary_foreground: "274 72% 25%",
        muted: "270 40% 96%",
        muted_foreground: "0 0% 45%",
        accent: "160 84% 39%",
        accent_foreground: "0 0% 98%",
        destructive: "0 84% 60%",
        destructive_foreground: "0 0% 98%",
        border: "270 40% 90%",
        input: "270 40% 90%",
        ring: "271 81% 56%",
    };

    pub const DARK: Self = Self {
        background: "274 72% 10%",
        foreground: "0 0% 98%",
        card: "274 72% 10%",
        card_foreground: "0 0% 98%",
        popover: "274 72% 10%",
        popover_foreground: "0 0% 98%",
        primary: "270 95% 75%",
        primary_foreground: "274 72% 12%",
        secondary: "274 50% 18%",
        secondary_foreground: "270 100% 95%",
        muted: "274 40% 18%",
        muted_foreground: "0 0% 64%",
        accent: "160 72% 45%",
        accent_foreground: "0 0% 98%",
        destructive: "0 63% 31%",
        destructive_foreground: "0 0% 98%",
        border: "274 40% 22%",
        input: "274 40% 22%",
        ring: "270 95% 75%",
    };

    #[must_use]
    pub fn hsl_primary(self) -> String {
        format!("hsl({})", self.primary)
    }
}

#[must_use]
pub fn css_hsl(space_separated: &str) -> String {
    format!("hsl({space_separated})")
}
