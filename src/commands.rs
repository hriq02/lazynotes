use std::{collections::HashMap, fs::read_to_string, io::Read, process::Command};

pub fn help(){
    println!("the following commands are availiable:");
    println!("-e   or --editor          opens with specific editor");
    println!("-a   or --add             to add path to te list ");
    println!("-l   or --list            open a list with all the paths");
    println!("-p   or --print           prints out the entire file content");
    println!("-gp  or --get_path        gets the path from the file");
    println!("-de  or --default_editor  sets the default editor");
    println!("-dp  or --default_path    sets the default path");
}


pub fn user_said_yes() -> bool{
    loop {
        let mut input = [0];
        let _ = std::io::stdin().read(&mut input);

        match input[0] as char {
            'y' | 'Y' => return true,
            'n' | 'N' => return false,
            _ => println!("y/n only please."),
        }
    }

}
pub fn go_to(path : &str, bash : &str){
    Command::new(bash)
                            .args(["cd", path])
                            .spawn()
                            .expect("failed to go to path");
}

pub fn list(){
    let path_file = crate::file_utils::current_exe_path() + "paths";
    println!("{}", crate::file_utils::get_file_content(&path_file));
}

pub fn get_path(key : &str, paths_map : &HashMap<String, String>){
    if paths_map.contains_key(key){
        println!("{}", paths_map.get( key ).unwrap().to_string() );
    }else{
        println!("{} does not contain in the list, please add it", key);
        println!();
        help();
    }
}

/// it is inspired on bat from sharkdp, chekout here -> https://github.com/sharkdp/bat
pub fn print_file(file_path : &str){
    let mut line_position : usize = 0;
    let file_name : String = crate::file_utils::get_file_name(file_path);

    println!("{:─<8}┬{:─<95}", "","");
    println!(  "{:8}│ File : {file_name}","");
    println!("{:─<8}┼{:─<95}", "","");
    
    for line in read_to_string(file_path).unwrap().lines(){
        line_position += 1;
        println!("{line_position:7} │ {line}");
    }
    
    println!("{:─<8}┴{:─<95}", "","");
}


