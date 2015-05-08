
#[cfg(any(target_os = "linux", target_os = "android"))]
pub mod epoll;

#[cfg(any(target_os = "macos", target_os = "ios", target_os = "freebsd"))]
pub mod event;

// TODO: switch from feature flags to conditional builds
#[cfg(feature = "eventfd")]
pub mod eventfd;

#[cfg(not(any(target_os = "ios", target_os = "freebsd")))]
pub mod ioctl;

pub mod signal;

pub mod socket;

pub mod stat;

#[cfg(any(target_os = "linux", target_os = "android"))]
pub mod syscall;

#[cfg(not(target_os = "ios"))]
pub mod termios;

#[cfg(any(target_os = "linux", target_os = "android"))]
pub mod utsname;

pub mod wait;

pub mod mman;

pub mod uio;

pub mod time;
