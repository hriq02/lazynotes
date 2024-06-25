use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::LineWriter;
use std::io::Read;
use std::io::Write;
use std::path::Path;


//read a specific line from a file
pub fn read_line(line: usize, file_path : &str) -> String {
    let file = File::open(file_path).expect("failed to open config file");
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
pub fn insert_to_file(file_path : &str, content : &str){
    let mut file = File::options().append(true).open(file_path).expect("failed to open file");
    writeln!(&mut file, "\n{}", content).expect("failed to write in the file");
}

pub fn get_file_content(file_path : &str) -> String{
    let mut file = File::open(file_path).expect("failed to open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("failed to read file");
    return content;
}

pub fn write_in_line(file_path : &str, line_content : String, line : usize){
    let mut file = File::create(file_path).expect("failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("failed to read file");
    let mut lines : Vec<String> = contents.lines().map(|line| line.to_string()).collect();
    lines[line] = line_content;
    file.write_all(lines.join("\n").as_bytes()).expect("failed to write in the file");
}

pub fn check_files_exist(){
    File::create("config").expect("was not able to create config file");
    File::create("paths").expect("was not able to create note_files file");
    if !Path::new("/notes").exists(){
        fs::create_dir_all("/notes").expect("was not able to create notes folder");
        insert_to_file("paths", "/notes");
    }
}