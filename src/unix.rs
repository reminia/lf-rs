use std::process::Command;

pub fn open_folder_and_select_items(paths: &[&str]) -> bool {
    if paths.len() > 1 {
        println!("unix supports to select only 1 file.")
    }
    let mut cmd = Command::new("nautilus");
    cmd.arg("-s").arg(paths[0]);
    let result = cmd
        .output()
        .expect("Fail to execute nautilus");
    // debug cmd
    if !result.status.success() {
        if let Some(stderr) = String::from_utf8(result.stderr).ok() {
            println!("stderr: {}", stderr);
        }
    }
    result.status.success()
}
