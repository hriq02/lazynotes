use std::{fs::File, io::Write};

static PATHS : &str = "paths";
static CONFIG : &str = "config";

pub fn read_config(line : usize) -> String{
    let config_path = crate::file_utils::current_exe_path() + CONFIG;

    return crate::file_utils::read_line(line, config_path.as_str())
}


pub fn change_config(config : &str, line : usize){
    let path : String = if line == 0 {config.to_string()} else{ read_config(0) };
    let editor : String = if line == 1 {config.to_string()} else{ read_config(1) };

    let final_config = path + "\n" + editor.as_str();
    
    let mut file = File::create(crate::file_utils::current_exe_path() + CONFIG).unwrap();
    file.write_all(final_config.as_bytes()).unwrap();
}


pub fn add_entry(path : &str){
    crate::file_utils::insert_to_file((crate::file_utils::current_exe_path() + PATHS).as_str(), &path);
}


pub fn create_file_in_default_path(file : String){
    let curr_path : String = crate::file_utils::current_exe_path();
    let default_path = crate::file_utils::read_line(0, (curr_path.clone() + CONFIG).as_str()) + "\\" + file.as_str();
    
    File::create(&default_path).expect("failed to create file");
    crate::file_utils::insert_to_file( (curr_path.clone()  + PATHS).as_str() ,&default_path );
}
