use std::process::{Command, Output};

pub fn open_folder_and_select_items(paths: &[&str]) -> Output {
    if paths.len() > 1 {
        println!("windows supports to select only 1 file.")
    }
    let mut cmd = Command::new("explorer.exe");
    let arg = format!("/select,{}", paths[0]);
    cmd.arg(arg);
    cmd.output().expect("failed to run explorer.exe")
}
