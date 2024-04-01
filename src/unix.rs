use std::process::{Command, Output};

pub fn open_folder_and_select_items(paths: &[&str]) -> Output {
    if paths.len() > 1 {
        println!("unix supports to select only 1 file.")
    }
    let mut cmd = Command::new("nautilus");
    cmd.arg("-s").arg(paths[0]);
    cmd.output().expect("failed to execute nautilus")
}
