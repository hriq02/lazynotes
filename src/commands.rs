use std::{collections::HashMap, io::Read, process::Command};

pub fn help(){
    println!("the following commands are availiable:");
    println!("-e   or --editor          opens with specific editor");
    println!("-a   or --add             to add path to te list ");
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