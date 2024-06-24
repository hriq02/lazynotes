use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;


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
