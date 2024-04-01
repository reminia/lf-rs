use std::process::Command;

pub fn open_folder_and_select_items(paths: &[&str]) -> bool {
    if paths.len() > 1 {
        println!("windows supports to select only 1 file.")
    }
    let mut cmd = Command::new("explorer.exe");
    let arg = format!("/select,{}", paths.join(","));
    cmd.arg(arg);
    let result = cmd.output().expect("failed to run explorer.exe");

    result.status.success()
}
