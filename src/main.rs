use std::{env, fs, io};

mod macos;

fn filter_files(file: &str) -> io::Result<String> {
    let entries = fs::read_dir(".")?;
    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();
        if file_name.to_lowercase().contains(&file.to_lowercase()) {
            return Ok(file_name.to_string());
        }
    }
    Ok("".to_string())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("No file specified, open current directory");
        macos::open_folder_and_select_items(&["."]);
    } else {
        let file_name = args.get(1).unwrap();
        let result = filter_files(file_name);
        match result {
            Ok(file) if file.is_empty() => println!("not found {}", file_name),
            Ok(file) => {
                println!("Found {}", &file);
                macos::open_folder_and_select_items(&[file.as_str()]);
            }
            Err(err) => eprintln!("Failed: {}", err)
        }
    }
}
