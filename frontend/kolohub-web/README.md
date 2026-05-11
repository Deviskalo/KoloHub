# kolohub-web

Dioxus frontend app for KoloHub.

## Platforms enabled

`Cargo.toml` uses target-specific Dioxus dependencies:

- `wasm32` target: `dioxus` with `web`
- non-`wasm32` targets: `dioxus` with `desktop,mobile`

This avoids wasm builds pulling desktop crates (`tao`) or unsupported tokio feature sets.

## Run commands

From this folder:

```bash
cd frontend/kolohub-web
```

### Web

```bash
dx serve --platform web
```

### Desktop

```bash
dx serve --platform desktop
```

### Android

```bash
dx serve --platform android
```

### iOS (macOS only)

```bash
dx serve --platform ios
```

## Notes

- If `dx` is missing: `cargo install dioxus-cli --locked`
- Web builds require: `rustup target add wasm32-unknown-unknown`
- Desktop/mobile may require platform SDK dependencies (GTK/WebKit, Android SDK/NDK, Xcode).
- Theme mode persistence (Phase 4.1) uses `localStorage` key: `kolohub.theme`.
