use std::process::Command;

fn apple_script(files: &[&str]) -> String {
    format!(
        "tell application \"Finder\"\n\
         activate\n\
         set selects to {{}}\n\
         {}\n\
         select selects\n\
         end tell",
        files
            .iter()
            .map(|file| format!("set end of selects to POSIX file \"{}\" as alias\n", file))
            .collect::<String>()
    )
}

pub fn open_folder_and_select_items(paths: &[&str]) -> bool {
    let script = apple_script(paths);
    let script_args = vec!["-e", &script];
    let result = Command::new("osascript")
        .args(&script_args)
        .output()
        .expect("Failed to execute AppleScript");

    result.status.success()
}
