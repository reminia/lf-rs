#[cfg(target_os = "windows")]
use windows as os;

#[cfg(target_os = "macos")]
use macos as os;

#[cfg(any(
target_os = "linux",
target_os = "freebsd",
target_os = "dragonfly",
target_os = "netbsd",
target_os = "openbsd",
target_os = "illumos",
target_os = "solaris"
))]
use unix as os;

pub fn open(paths: &[&str]) -> bool {
    os::open_folder_and_select_items(paths)
}

#[cfg(not(any(
target_os = "linux",
target_os = "freebsd",
target_os = "dragonfly",
target_os = "netbsd",
target_os = "openbsd",
target_os = "illumos",
target_os = "solaris",
target_os = "ios",
target_os = "macos",
target_os = "windows",
target_os = "haiku",
target_os = "redox"
)))]
compile_error!("open is not supported on this platform");

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(any(
target_os = "linux",
target_os = "freebsd",
target_os = "dragonfly",
target_os = "netbsd",
target_os = "openbsd",
target_os = "illumos",
target_os = "solaris"
))]
mod unix;
