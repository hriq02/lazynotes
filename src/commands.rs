use std::{collections::HashMap, fs::File, io::Read};



pub fn add_file(paths_map : &HashMap<String, String> ,new_file_path : &str){
    if !paths_map.contains_key(new_file_path){
        if new_file_path.contains('/'){
            crate::file_utils::insert_to_file( "config",&new_file_path );
        }else{
            print!("file is not a path, be sure that is a path: {}", new_file_path);
            return;
        }
    }else{
        println!("file already exist in the list: {}", paths_map.get(new_file_path).unwrap().to_string() );
        return;
    }
}

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


pub fn create_file_in_default_path(file : String){
    let default_path = crate::file_utils::read_line(0, "config") + "\\" + file.as_str();
    File::create(&default_path).expect("failed to create file");
    crate::file_utils::insert_to_file( "paths",&default_path );
}
