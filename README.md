# gpui-binder nightly

This branch is generated automatically from:

- GPUI/Zed source: `zed-industries/zed` at `main`
- gpui-component source: `longbridge/gpui-component` at `main`

The dependency revision below intentionally points to the generated source commit before this README commit:

```toml
# Pick the target feature set you actually build. Examples:
# macOS desktop: features = ["gpui_macros", "gpui_macos", "font-kit", "runtime_shaders"]
# Linux desktop: features = ["gpui_macros", "gpui_linux", "x11", "wayland", "font-kit", "runtime_shaders"]
# Windows desktop: features = ["gpui_macros", "gpui_windows", "font-kit", "runtime_shaders", "windows-manifest"]
# Web/WASM: features = ["gpui_macros", "gpui_web"]
gpui = { git = "https://github.com/gpui-binder/gpui-binder", package = "gpui_facade", rev = "760315a9989bd2bd8aa9d7a846edaa6b0110cdd8", default-features = false, features = ["gpui_macros"] }
gpui-component = { git = "https://github.com/gpui-binder/gpui-binder", package = "gpui-component", rev = "760315a9989bd2bd8aa9d7a846edaa6b0110cdd8" }
```

Generated branches:

- `generated-zed-nightly`: Zed/GPUI fork branch before importing gpui-component
- `generated-gpui-nightly`: final branch after importing gpui-component and passing Cargo local dependency checks
