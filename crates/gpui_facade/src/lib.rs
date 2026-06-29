#![doc = "Facade crate that re-exports GPUI plus optional GPUI-related crates."]

pub use gpui::*;

// No extra test re-export is needed here. When the facade feature
// `test-support` is enabled, it enables `gpui/test-support`, and upstream
// `gpui` itself publicly re-exports `test::*` from `gpui::*`.

pub mod gpui_platform {
    pub use ::gpui_platform::*;
}

#[cfg(any(feature = "gpui_macros", feature = "inspector"))]
pub mod gpui_macros {
    pub use ::gpui_macros::*;
}

#[cfg(any(feature = "gpui_macros", feature = "inspector"))]
pub use ::gpui_macros::*;

#[cfg(all(target_family = "wasm", feature = "gpui_web"))]
pub mod gpui_web {
    pub use ::gpui_web::*;
}

#[cfg(all(target_family = "wasm", feature = "gpui_web"))]
pub use ::gpui_web::*;

#[cfg(all(target_os = "macos", feature = "gpui_macos"))]
pub mod gpui_macos {
    pub use ::gpui_macos::*;
}

#[cfg(all(target_os = "macos", feature = "gpui_macos"))]
pub use ::gpui_macos::*;

#[cfg(all(any(target_os = "linux", target_os = "freebsd"), feature = "gpui_linux"))]
pub mod gpui_linux {
    pub use ::gpui_linux::*;
}

#[cfg(all(any(target_os = "linux", target_os = "freebsd"), feature = "gpui_linux"))]
pub use ::gpui_linux::*;

#[cfg(all(target_os = "windows", feature = "gpui_windows"))]
pub mod gpui_windows {
    pub use ::gpui_windows::*;
}

#[cfg(all(target_os = "windows", feature = "gpui_windows"))]
pub use ::gpui_windows::*;
