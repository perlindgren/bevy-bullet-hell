#[cfg(target_os = "windows")]
mod dev_input_windows;
#[cfg(target_os = "windows")]
pub use dev_input_windows::*;

#[cfg(not(target_os = "windows"))]
mod dev_input_unix;
#[cfg(not(target_os = "windows"))]
pub use dev_input_unix::*;
