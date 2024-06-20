//use std::io::prelude::*;
//use std::string;
use std::env;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
//use std::path::Path;
//use std::io::Error;
use std::process::Command;

fn main() {
    let notes_path = read_config(0);
    let editor = read_config(1);
    let bash = if env::consts::OS == "windows" { "powershell" } else if env::consts::OS == "macos" { "terminal" } else { "bash" };

    let mut cmd = Command::new(bash);
    cmd.args([editor.clone(), format!("{}\\nana", notes_path)]);

    //will iterate over the files in the directory and map them
    if let Ok(paths) = fs::read_dir(&notes_path) {
        for path in paths {
            if let Ok(entry) = path {
                println!("Name: {:?}", entry.file_name());
            }
        }
    }

    // Execute the command oppening the editor
    match cmd.output() {
        Ok(o) => {
            unsafe {
                println!("{}", String::from_utf8_unchecked(o.stdout));
            }
        }
        Err(e) => {
            println!("Error executing command: {}", e);
        }
    }
}


fn read_config(line: usize) -> String {
    let file = File::open("config").expect("failed to open config file");
    let content = BufReader::new(&file);
    let mut lines = content.lines();

    // Skip to the desired line
    let line_content = lines.nth(line).expect("no lines found in this position");

    // Convert the line content to a String
    match line_content {
        Ok(lc) => lc,
        Err(_) => String::from(""),
    }
}
