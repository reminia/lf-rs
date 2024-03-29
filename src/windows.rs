use std::process::Command;

pub fn open_folder_and_select_items(paths: &[&str]) -> bool {
    let mut cmd = Command::new("explorer.exe");
    let arg = format!("/select,{}", paths.join(","));
    cmd.arg(arg);
    let result = cmd
        .output()
        .expect("Failed to run explorer.exe command");

    result.status.success()
}