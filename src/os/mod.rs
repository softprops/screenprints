#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use self::windows::*;

#[cfg(not(windows))]
mod default;
#[cfg(not(windows))]
pub use self::default::*;
