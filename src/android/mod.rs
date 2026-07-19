#[cfg(target_os = "android")]
pub mod jni;

#[cfg(target_os = "android")]
pub mod shizuku;

#[cfg(target_os = "android")]
pub mod accessibility;

#[cfg(target_os = "android")]
pub mod foreground;

#[cfg(target_os = "android")]
pub mod notification;

#[cfg(target_os = "android")]
pub mod clipboard;

#[cfg(target_os = "android")]
pub mod webview;
