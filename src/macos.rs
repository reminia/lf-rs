use std::fmt::Write;
use std::process::{Command, Output};

fn apple_script(files: &[&str]) -> String {
    format!(
        "tell application \"Finder\"\n\
         activate\n\
         set selects to {{}}\n\
         {}\n\
         select selects\n\
         end tell",
        files.iter().fold(String::new(), |mut acc, file| {
            writeln!(
                &mut acc,
                "set end of selects to POSIX file \"{}\" as alias",
                file
            )
            .unwrap();
            acc
        })
    )
}

pub fn open_folder_and_select_items(paths: &[&str]) -> Output {
    let script = apple_script(paths);
    let script_args = vec!["-e", &script];
    Command::new("osascript")
        .args(&script_args)
        .output()
        .expect("failed to execute AppleScript")
}
