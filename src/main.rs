use std::{env, fs, io};

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
        lf::open(&["."]);
    } else {
        let file_name = args.get(1).unwrap();
        let result = filter_files(file_name);
        match result {
            Ok(file) if file.is_empty() => println!("Not found {}", file_name),
            Ok(file) => {
                println!("Found {}", &file);
                lf::open(&[file.as_str()]);
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}
