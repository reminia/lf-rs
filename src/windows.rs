use std::process::Command;

pub fn open_folder_and_select_items(paths: &[&str]) -> bool {
    let mut cmd = Command::new("explorer");
    cmd.arg("/select,");
    for path in paths {
        cmd.arg(path);
    }
    cmd.output().unwrap().status
}
