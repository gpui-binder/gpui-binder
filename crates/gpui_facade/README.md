# gpui_facade

A facade crate that re-exports `gpui`, exposes `gpui_platform`, and can optionally expose related GPUI packages.

## Example

```toml
gpui = { package = "gpui_facade", workspace = true }
```

```rust
use gpui::*;

let app = gpui::gpui_platform::application();
```

## Optional package re-exports

```toml
gpui = { package = "gpui_facade", workspace = true, features = ["gpui_web"] }
```

```rust
use gpui::*;

#[cfg(target_family = "wasm")]
let _ = gpui::gpui_web::WebPlatform::new(true);
```
