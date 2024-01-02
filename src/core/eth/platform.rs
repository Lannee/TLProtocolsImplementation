

#[cfg(any(target_os = "linux", target_os = "android"))]
#[path = "platform/linux.rs"]
pub mod linux;


#[cfg(target_os = "windows")]
#[path = "platform/win.rs"]
pub mod win;

mod sys {
    #[cfg(any(target_os = "android", target_os = "linux"))]
    pub use super::linux::EthSocket as EthSocket;

    // #[cfg(windows)]
    // pub use super::win::EthSocket as EthSocket;
}

pub use sys::EthSocket;