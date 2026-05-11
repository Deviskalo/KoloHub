//! Raw scales — Tailwind-aligned hex. Source of truth: `tokens.css`.

/// Purple scale (`--color-purple-*`).
pub mod purple {
    pub const P50: &str = "#faf5ff";
    pub const P100: &str = "#f3e8ff";
    pub const P200: &str = "#e9d5ff";
    pub const P300: &str = "#d8b4fe";
    pub const P400: &str = "#c084fc";
    pub const P500: &str = "#a855f7";
    pub const P600: &str = "#9333ea";
    pub const P700: &str = "#7e22ce";
    pub const P800: &str = "#6b21a8";
    pub const P900: &str = "#581c87";
    pub const P950: &str = "#3b0764";
}

/// Gray scale (`--color-gray-*`).
pub mod gray {
    pub const G50: &str = "#f9fafb";
    pub const G100: &str = "#f3f4f6";
    pub const G200: &str = "#e5e7eb";
    pub const G300: &str = "#d1d5db";
    pub const G400: &str = "#9ca3af";
    pub const G500: &str = "#6b7280";
    pub const G600: &str = "#4b5563";
    pub const G700: &str = "#374151";
    pub const G800: &str = "#1f2937";
    pub const G900: &str = "#111827";
    pub const G950: &str = "#030712";
}

/// Emerald accent (`--color-emerald-*` + Tailwind 400 for dark hover).
pub mod emerald {
    pub const E50: &str = "#ecfdf5";
    pub const E100: &str = "#d1fae5";
    pub const E400: &str = "#34d399";
    pub const E500: &str = "#10b981";
    pub const E600: &str = "#059669";
    pub const E700: &str = "#047857";
}

/// Absolute whites used in tokens.
pub mod base {
    pub const WHITE: &str = "#ffffff";
    pub const ON_BRAND: &str = "#ffffff";
}
