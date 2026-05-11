//! Resolved semantic colors for light and dark — mirrors `tokens.css` `:root` / `.dark`.

use crate::layout::Shadow;
use crate::palette::{base, emerald, gray, purple};
use crate::hsl::ShadcnSlots;

/// Active color scheme for Dioxus state (`use_signal(|| ColorScheme::Light)`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ColorScheme {
    #[default]
    Light,
    Dark,
}

impl ColorScheme {
    #[must_use]
    pub fn shadcn(self) -> ShadcnSlots {
        match self {
            Self::Light => ShadcnSlots::LIGHT,
            Self::Dark => ShadcnSlots::DARK,
        }
    }

    #[must_use]
    pub fn toggle(self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }

    /// `class` token for `tokens.css` — set on `<html>` or root `div`.
    #[must_use]
    pub fn html_class(self) -> &'static str {
        match self {
            Self::Light => "",
            Self::Dark => "dark",
        }
    }

    // --- Page & surfaces ---

    #[must_use]
    pub fn page_gradient_from(self) -> &'static str {
        match self {
            Self::Light => purple::P50,
            Self::Dark => purple::P950,
        }
    }

    #[must_use]
    pub fn page_gradient_to(self) -> &'static str {
        match self {
            Self::Light => purple::P100,
            Self::Dark => purple::P900,
        }
    }

    /// Full CSS `background` for the app shell (Iskalo-style gradient).
    #[must_use]
    pub fn page_background_css(self) -> String {
        format!(
            "linear-gradient(to bottom right, {}, {})",
            self.page_gradient_from(),
            self.page_gradient_to()
        )
    }

    #[must_use]
    pub fn surface(self) -> &'static str {
        match self {
            Self::Light => base::WHITE,
            Self::Dark => gray::G950,
        }
    }

    #[must_use]
    pub fn surface_elevated(self) -> &'static str {
        match self {
            Self::Light => base::WHITE,
            Self::Dark => gray::G900,
        }
    }

    #[must_use]
    pub fn surface_glass(self) -> &'static str {
        match self {
            Self::Light => "rgba(255, 255, 255, 0.8)",
            Self::Dark => "rgba(3, 7, 18, 0.8)",
        }
    }

    #[must_use]
    pub fn border_subtle(self) -> &'static str {
        match self {
            Self::Light => purple::P100,
            Self::Dark => purple::P800,
        }
    }

    #[must_use]
    pub fn border_strong(self) -> &'static str {
        match self {
            Self::Light => purple::P200,
            Self::Dark => purple::P700,
        }
    }

    // --- Text ---

    #[must_use]
    pub fn text_primary(self) -> &'static str {
        match self {
            Self::Light => gray::G900,
            Self::Dark => gray::G50,
        }
    }

    #[must_use]
    pub fn text_secondary(self) -> &'static str {
        match self {
            Self::Light => gray::G600,
            Self::Dark => gray::G300,
        }
    }

    #[must_use]
    pub fn text_muted(self) -> &'static str {
        match self {
            Self::Light => gray::G500,
            Self::Dark => gray::G400,
        }
    }

    #[must_use]
    pub const fn text_on_brand(self) -> &'static str {
        base::ON_BRAND
    }

    // --- Brand (purple) ---

    #[must_use]
    pub fn brand(self) -> &'static str {
        match self {
            Self::Light => purple::P600,
            Self::Dark => purple::P500,
        }
    }

    #[must_use]
    pub fn brand_hover(self) -> &'static str {
        match self {
            Self::Light => purple::P700,
            Self::Dark => purple::P400,
        }
    }

    #[must_use]
    pub fn brand_muted(self) -> &'static str {
        match self {
            Self::Light => purple::P100,
            Self::Dark => purple::P900,
        }
    }

    #[must_use]
    pub fn brand_foreground(self) -> &'static str {
        match self {
            Self::Light => purple::P900,
            Self::Dark => purple::P100,
        }
    }

    #[must_use]
    pub fn gradient_text_from(self) -> &'static str {
        match self {
            Self::Light => purple::P600,
            Self::Dark => purple::P400,
        }
    }

    #[must_use]
    pub fn gradient_text_to(self) -> &'static str {
        match self {
            Self::Light => purple::P400,
            Self::Dark => purple::P300,
        }
    }

    /// CSS `background` for gradient text (clip in CSS / web).
    #[must_use]
    pub fn gradient_text_background_css(self) -> String {
        format!(
            "linear-gradient(to right, {}, {})",
            self.gradient_text_from(),
            self.gradient_text_to()
        )
    }

    // --- Commerce accent (emerald) ---

    #[must_use]
    pub fn accent(self) -> &'static str {
        match self {
            Self::Light => emerald::E600,
            Self::Dark => emerald::E500,
        }
    }

    #[must_use]
    pub fn accent_hover(self) -> &'static str {
        match self {
            Self::Light => emerald::E700,
            Self::Dark => emerald::E400,
        }
    }

    #[must_use]
    pub fn accent_muted(self) -> &'static str {
        match self {
            Self::Light => emerald::E100,
            Self::Dark => "rgba(16, 185, 129, 0.15)",
        }
    }

    // --- Chrome ---

    #[must_use]
    pub fn shadow_nav(self) -> &'static str {
        match self {
            Self::Light => Shadow::NAV_LIGHT,
            Self::Dark => Shadow::NAV_DARK,
        }
    }

    #[must_use]
    pub fn shadow_card(self) -> &'static str {
        match self {
            Self::Light => Shadow::CARD_LIGHT,
            Self::Dark => Shadow::CARD_DARK,
        }
    }

    /// Outline button border (light uses gray-200; dark uses purple border subtle).
    #[must_use]
    pub fn outline_button_border(self) -> &'static str {
        match self {
            Self::Light => gray::G200,
            Self::Dark => purple::P800,
        }
    }
}
