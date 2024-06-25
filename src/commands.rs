use std::collections::HashMap;



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
    println!("those following commands are availiable:");
    println!("-e   or --editor          opens with specific editor");
    println!("-a   or --add             to add path to te list ");
    println!("-p   or --print           prints out the entire file content");
    println!("-gp  or --get_path        gets the path from the file");
    println!("-de  or --default_editor  sets the default editor");
    println!("-dp  or --default_path    sets the default path");
}