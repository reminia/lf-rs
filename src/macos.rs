use std::process::Command;

// leverage AppleScript to focus on the file
pub fn open_folder_and_select_items(folder_path: &str, item_paths: &[&str]) -> bool {
  let mut script = String::new();
  script.push_str("tell application \"Finder\"\n");
  script.push_str(&format!("activate\n"));
  // script.push_str(&format!("select POSIX file \"{}\" as alias\n", folder_path));
  for item_path in item_paths {
      script.push_str(&format!("select POSIX file \"{}\" as alias\n", item_path));
  }
  script.push_str("end tell\n");

  let script_args = vec!["-e", &script];
  let result = Command::new("osascript")
      .args(&script_args)
      .output()
      .expect("Failed to execute AppleScript");

  result.status.success()
}
