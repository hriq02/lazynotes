//use std::io::prelude::*;
//use std::string;
use std::env;
// use std::fs;
// use std::fs::read_to_string;
//use std::path::Path;
//use std::io::Error;
use std::process::Command;

mod file_utils;
mod mapping;
mod commands;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut editor = file_utils::read_line(1,"config");
    let bash = if env::consts::OS == "windows" { "powershell" } else if env::consts::OS == "macos" { "terminal" } else { "bash" };

    let mut cmd = Command::new(bash);
    let paths_map = mapping::read_notes_file("paths");
    let mut path_ = file_utils::read_line(0, "config");

    //------------------------------------------------------------------------------------------------------------------------------
    
    if args.len() <= 1 {
        println!("you need to specify a file or a path to a project");
        commands::help();
        return;
    }
    let mut arg_n = 1;
    while arg_n < args.len() {
        match args[arg_n].as_str() {
            "-e" | "--editor" => {
                editor = args[arg_n + 1].clone();
                arg_n += 1;
            }
            "-a" | "--add" => {
                commands::add_file(&paths_map,&args[arg_n + 1]);
                return;
            }
            "-gp" | "--get_path" => {
                println!("{}", paths_map.get( &args[arg_n + 1].clone() ).unwrap().to_string() );
                return;
            }
            "-de" | "--default_editor" => {
                file_utils::write_in_line("config",args[arg_n + 1].clone(), 1);
                return;
            }
            "-dp" | "--default_path" => {
                file_utils::write_in_line("config",args[arg_n + 1].clone(), 0);
                return;
            }
            "-p" | "--print" =>{
                println!("{}", file_utils::get_file_content(paths_map.get( &args[arg_n + 1].clone() ).unwrap().to_string().as_str()));
                return;
            }
            "-h" | "--help" => {
                commands::help();
                return;
            }
            _ => {
                if paths_map.contains_key(&args[arg_n]) {
                    path_ = paths_map.get( &args[arg_n].clone() ).unwrap().to_string();
                }else{
                    path_ = path_ + "/" + &args[arg_n].clone();
                }
            }
        }
        arg_n += 1;
    }
    //------------------------------------------------------------------------------------------------------------------------------

    println!("filePath: {}", path_ );
    cmd.args([editor.clone(), format!("{}", path_ )]);

    // Execute the command oppening the editor and print out the result
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

