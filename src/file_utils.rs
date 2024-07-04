use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::Path;


pub fn read_line(line: usize, file_path : &str) -> String {
    let file = File::open(file_path).expect("failed to open config file");
    let content = BufReader::new(&file);
    let mut lines = content.lines();

    let line_content = lines.nth(line).expect("no lines found in this position");

    match line_content {
        Ok(lc) => lc,
        Err(_) => String::from(""),
    }
}


pub fn insert_to_file(file_path : &str, content : &str){
    let mut file = File::options().append(true).open(file_path).expect("failed to open file");

    writeln!(&mut file, "{}", content).expect("failed to write in the file");
}


pub fn get_file_content(file_path : &str) -> String{
    let mut file = File::open(file_path).expect("failed to open file");
    let mut content = String::new();

    file.read_to_string(&mut content).expect("failed to read file");

    return content;
}


pub fn check_files_exist(){
    let curr_path : String = current_exe_path();

    let paths_file = curr_path.clone() + "paths";
    if !Path::new(&paths_file).exists(){
        File::create(paths_file).expect("was not able to create note_files file");
    }

    let notes_file = curr_path.clone() + "notes";
    if !Path::new(&notes_file).exists(){
        fs::create_dir_all(notes_file.clone()).expect("was not able to create notes folder");
    }

    let config_file = curr_path.clone() + "config";
    if !Path::new(&config_file).exists(){
        let mut new_conifg_file = File::create(config_file).expect("was not able to create config file");
        let text : String = notes_file.clone() + "\nnotepad";
        new_conifg_file.write_all(&text.as_bytes()).expect("failed to write in the file");
    }

}


pub fn current_exe_path() -> String{
    let exe_path : String = std::env::current_exe() .unwrap().to_str().unwrap().to_string();

    let curr_path : String = match exe_path.strip_suffix("lzn.exe") {
        Some(exe_path) => exe_path.to_string(),
        None => exe_path.to_string()
    };

    curr_path
}
