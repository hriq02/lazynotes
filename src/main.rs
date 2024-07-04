use std::env;
use std::path::Path;
use std::process::Command;

mod file_utils;
mod mapping;
mod commands;
mod config;

static DEFAULT_PATH : usize = 0;
static DEFAULT_EDITOR : usize = 1;


fn main() {
    let args: Vec<String> = env::args().collect();
    
    file_utils::check_files_exist();

    let current_path : String = file_utils::current_exe_path();
    
    let mut editor = file_utils::read_line(DEFAULT_EDITOR,(current_path.clone() + "config").as_str() );
    let bash = if env::consts::OS == "windows" { "powershell" } else if env::consts::OS == "macos" { "terminal" } else { "bash" };

    let paths_map = mapping::read_notes_file((current_path.clone() + "paths").as_str());
    let mut path_ = String::new();

    //------------------------------------------------------------------------------------------------------------------------------
    

    if args.len() <= 1 {
        println!("you need to specify a file or a path to a project");
        commands::help();
        return;
    }
    let mut arg_n = 1;
    let mut is_to_print : bool = false;
    let mut is_to_go : bool = false;
    
    while arg_n < args.len() {
        match args[arg_n].as_str() {
            "-e" | "--editor" => {
                editor = args[arg_n + 1].clone();
                arg_n += 1;
            }
            "-p" | "--print" =>{
                is_to_print = true;
            }
            "-a" | "--add" => {
                config::add_entry(&args[arg_n + 1]);

                println!("file add to the list, open it?");
                if !commands::user_said_yes() {return;}

                path_ = args[arg_n + 1].clone();
            }
            "-gp" | "--get_path" => {
                commands::get_path(&args[arg_n + 1], &paths_map);
                return;
            }
            "-de" | "--default_editor" => {
                config::change_config(&args[arg_n + 1],DEFAULT_EDITOR);
                return;
            }
            "-dp" | "--default_path" => {
                config::change_config(&args[arg_n + 1],DEFAULT_PATH);
                return;
            }
            "-h" | "--help" => {
                commands::help();
                return;
            }
            "-l" | "--list" => {
                commands::list();
                return;
            }
            "-go" | "--go_to" =>{
                is_to_go = true;
            }
            _ => {
                if paths_map.contains_key(&args[arg_n]) {
                    path_ = paths_map.get( &args[arg_n].clone() ).unwrap().to_string();
                }else{
                    path_ = args[arg_n].clone();
                }
            }
        }
        arg_n += 1;
    }
    //------------------------------------------------------------------------------------------------------------------------------

    if is_to_go{
        commands::go_to(&path_, bash);
        return;
    }

    if is_to_print {
        println!("{}",path_.clone());
        println!("{}", file_utils::get_file_content(&path_));
        return;
    }


    if !Path::new(&path_).exists(){
        println!("this file does not contain in the list, do you want to create this file with that name in default folder?");
        if !commands::user_said_yes() {return;}

        config::create_file_in_default_path(path_.clone());
        path_ = config::read_config(DEFAULT_PATH) + "\\" + &path_;
        config::add_entry(&path_);
    }
    let mut cmd = Command::new(bash);
    
    cmd .args([editor.clone(), format!("{}", path_ )])
        .spawn()
        .expect("failed to execute command")
        .wait()
        .expect("failed to wait for command");

    // match cmd.output() {
    //     Ok(o) => {
    //         unsafe {
    //             println!("{}", String::from_utf8_unchecked(o.stdout));
    //         }
    //     }
    //     Err(e) => {
    //         println!("Error executing command: {}", e);
    //     }
    // }
    
}

