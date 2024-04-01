use std::process::Output;
#[cfg(target_os = "windows")]
use windows as os;

#[cfg(target_os = "macos")]
use macos as os;

#[cfg(target_os = "linux")]
use unix as os;

pub fn open(paths: &[&str]) -> Output {
    os::open_folder_and_select_items(paths)
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows",)))]
compile_error!("open is not supported on this platform");

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "linux")]
mod unix;
