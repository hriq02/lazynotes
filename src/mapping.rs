use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::ErrorKind;



pub fn read_notes_file(path : &str) -> HashMap<String, String>{
    let mut all_notes = HashMap::new();

    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                panic!("File not found: {}", path);
            } else {
                panic!("Error opening file: {}", e);
            }
        }
    };
    let lines = BufReader::new(&file).lines();
    
    for line in lines {
        match line {
            Ok(ln) => {
                insert_new_note(&mut all_notes, &ln.to_string());
            }
            Err(_) => {}
        }
    }
    return all_notes;
}


pub fn insert_new_note( all_notes : &mut HashMap<String, String>, path : &String ) {
    let file_path = path.trim();
    if file_path.is_empty() {return;}
    
    if let Some(file_name) = file_path.rsplit_once(|c| c == '/' || c == '\\') {
        // let mut fl : String = file_name.1.to_string();

        // // if unique {
        // //     fl = fl[0..fl.find(".").unwrap_or(fl.len())].to_string();
        // // }

        // if all_notes.contains_key(&fl){
        //     println!("warning: {} already exists ", fl);
        //     // if unique { println!("warning: {} already exists as unique file", fl) }
        //     // else { println!("warning: {} already exists", fl) };
        //     return;
        // }
        print!("{} ", file_name.1);

        all_notes.insert(file_name.1.to_string(), path.to_string());
    }
}