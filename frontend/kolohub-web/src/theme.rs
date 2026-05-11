//! Theme persistence helpers — read/write `kolohub.theme` in `localStorage`.

use kolohub_theme::ColorScheme;

pub fn load_theme_from_storage() -> ColorScheme {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(value)) = storage.get_item("kolohub.theme") {
                    if value == "dark" {
                        return ColorScheme::Dark;
                    }
                }
            }
        }
    }
    ColorScheme::Light
}

pub fn save_theme_to_storage(scheme: ColorScheme) {
    #[cfg(not(target_arch = "wasm32"))]
    let _ = scheme;

    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let value = match scheme {
                    ColorScheme::Light => "light",
                    ColorScheme::Dark => "dark",
                };
                let _ = storage.set_item("kolohub.theme", value);
            }
        }
    }
}
