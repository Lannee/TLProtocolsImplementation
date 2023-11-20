mod linux;
mod win;

#[cfg(any(target_os = "linux", target_os = "android"))]
#[path = "platform/linux.rs"]
pub mod channel;


#[cfg(target_os = "windows")]
#[path = "platform/windows.rs"]
pub mod channel;