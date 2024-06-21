//use std::io::prelude::*;
//use std::string;
use std::env;
// use std::fs;
// use std::fs::read_to_string;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::ErrorKind;
//use std::path::Path;
//use std::io::Error;
use std::process::Command;
use std::collections::HashMap;

fn main() {
    // let notes_path = read_config(0);
    let editor = read_config(1);
    let bash = if env::consts::OS == "windows" { "powershell" } else if env::consts::OS == "macos" { "terminal" } else { "bash" };

    let mut cmd = Command::new(bash);
    let all_notes = read_notes_path("note_files");

    //println!("filePath: {}", all_notes.get("nana").unwrap() );

    //all_notes.contains_key("nana");
    
    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        // for arg in args{
        //     println!("argument: {}",&arg);
        // }
        
        return;
    }

    println!("filePath: {}", all_notes.get(&args[1]).unwrap() );

    cmd.args([editor.clone(), format!("{}", all_notes.get(&args[1]).unwrap() )]);
    //cmd.args([editor.clone(), format!("{}\\", notes_path)]);


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

fn read_notes_path(path : &str) -> HashMap<String, String>{
    let mut all_notes = HashMap::new();

    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                panic!("File not found: {}", path);
            } else {
                panic!("Error opening file: {}", e);
            }
        }
    };
    let lines = BufReader::new(&file).lines();
    
    for line in lines {
        match line {
            Ok(ln) => {
                if ln.trim().is_empty() {continue;}
                let file_path = ln.trim();
                if let Some(file_name) = file_path.rsplit_once(|c| c == '/' || c == '\\') {
                    all_notes.insert(get_file_name(&file_name.1.to_string()), ln.to_string());
                }
            }
            Err(_) => {}
        }
    }
    return all_notes;
}

fn get_file_name(path: &String) -> String {
    return path[0..path.find(".").unwrap_or(path.len())].to_string()
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
