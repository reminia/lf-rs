use std::process::Command;

pub fn open_folder_and_select_items(paths: &[&str]) -> bool {
    if paths.len() > 1 {
        println!("unix supports to select only 1 file.")
    }
    let cmd = Command::new("nautilus");
    let arg = format!("-s {}", paths.join(","));
    let result = cmd.arg(arg)
        .output()
        .expect("Fail to execute nautilus");
    result.status.success()
}
