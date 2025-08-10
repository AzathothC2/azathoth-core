/// Linux type definitions.
#[cfg(target_os = "linux")]
pub mod linux;

/// Windows type definitions.
#[cfg(target_os = "windows")]
pub mod windows;


/// Current platform type definitions.
/// This module re-exports the platform-specific modules based on the target OS.
/// It allows for easier access to the current platform's definitions without needing to
/// conditionally import the specific module in other parts of the codebase.
/// The `Current` module will point to either `linux` or `windows` based on the target OS.
#[cfg(target_os = "windows")]
pub use windows as Current;
#[cfg(target_os = "linux")]
pub use linux as Current;