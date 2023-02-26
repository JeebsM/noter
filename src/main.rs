use std::env;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;


#[derive(Debug)]
pub struct NoterPaths {
    home_path: String,
    note_path: String,
    todo_path: String,
}
#[derive(Debug)]
pub struct Note {
    path: String,
    title: String,
    category: String,
    content: String,
}
// note with 1 argument: write a thought to thoughts directory
// note with 2 arguments: write a task to todo directory
// note with 3 arguments: write a file to thoughts directory
fn main() -> std::io::Result<()> {
    let home_path: String = env::var("HOME").unwrap().to_owned();
    let note_path: &str = "/notes/";
    let todo_path: &str = "/notes/todo/";
    let absolute_note_path: String = home_path + note_path;
    let absolute_todo_path: String = home_path + todo_path;
    let note_directory = NoterPaths {
        home_path.clone(), 
        note_path : absolute_note_path, 
        todo_path : absolute_todo_path,
    };
    dbg!(&note_directory);
    // Always check if my-thoughts exist in $HOME
    
    // Count arguments list to define prog behavior
    let args: &Vec<String> = &env::args().collect();
    // dbg!(args);
    let arguments_count: usize = args.len();

    let note: Note = match arguments_count {
        1 => {
            println!("Not writing anything. I quit!");
            Note{
                path: String::from(""), 
                title:  String::from(""), 
                category: String::from(""), 
                content: String::from("")
            }
        }
        2 => {
            Note{
                path: note_path.to_owned(), 
                title: args[1].clone(), 
                category: String::from(""), 
                content: String::from("")
            }
        }
        _ => {
            Note{
                path: note_path.to_owned(), 
                title: args[1].clone(), 
                category: args[2].clone(), 
                content: String::from("")
            }
        }
    };

    dbg!(note);
    let file_exist: bool = Path::new(&note_directory.note_path).exists();

    if !file_exist {
        File::create(note_directory.note_path.clone()).expect("File does not exist.");
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(note_directory.note_path)
        .unwrap();

    if let Err(e) = writeln!(file, "{:?}", &note) {
        eprintln!("Couldn't write to file: {}", e);
    }

    Ok(())
}

// use std::fs::File;
// use std::io::BufReader;
// use std::io::prelude::*;

// fn main() -> std::io::Result<()> {
//     let file = File::open("foo.txt")?;
//     let mut buf_reader = BufReader::new(file);
//     let mut contents = String::new();
//     buf_reader.read_to_string(&mut contents)?;
//     assert_eq!(contents, "Hello, world!");
//     Ok(())
// }
