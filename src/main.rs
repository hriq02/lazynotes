//use std::io::prelude::*;
//use std::string;
//use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
//use std::path::Path;
use std::io::Error;
use std::process::Command;

fn main() {
    let notes_path = read_config(0);
    let bash = read_config(1);
    let editor = read_config(2);

    let mut cmd = Command::new(bash.unwrap());
    cmd.args([editor.unwrap(),notes_path.unwrap() + "\\nana"]);

    match cmd.output(){
        Ok(o) =>{
            unsafe{
                println!("{}",String::from_utf8_unchecked(o.stdout));
            }
        }
        Err(o) =>{
            println!("{}", o);
        }
    }
}


fn read_config(line : usize) -> Result<String, Error>{
    let file = File::open("config").expect("failed to open config file");
    let content = BufReader::new(&file);
    let mut lines = content.lines();
    lines.nth(line).expect("no lines found in this position")
}
