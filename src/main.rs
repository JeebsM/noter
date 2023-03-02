use std::{env, fs};
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
    let note_path: &str = "/my-thoughts/notes/";
    let todo_path: &str = "/my-thoughts/notes/todo/";
    let absolute_note_path: String = home_path.clone() + note_path;
    let absolute_todo_path: String = home_path.clone() + todo_path;
    let note_directory = NoterPaths {
        home_path : home_path.clone(),
        note_path : absolute_note_path,
        todo_path : absolute_todo_path,
    };
    // dbg!(&note_directory);
    // Always check if my-thoughts exist in $HOME
    let directory_creation_result = fs::create_dir_all(&todo_path);
    let directory_creation = match directory_creation_result {
        Ok(_) => println!("my-thougts directory was created"),
        Err(error) => println!("Directory creation failed: {:?}", error),
    };

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
            let note_is_todo: bool = args[2].contains("todo");
            let category: &str = match note_is_todo {
                true => "todo",
                _ => "note"
            };
            let file_path: String = match note_is_todo {
                true => {
                    note_directory.todo_path.clone()
                        + &args[1].replace(" ", "_")
                }
                _ => {
                    note_directory.note_path.clone()
                        + &args[1].replace(" ", "_")
                }
            };

            Note{
                path: file_path,
                title: args[1].clone(),
                category: category.to_string(),
                content:args[2].clone()
            }
        }
    };
    // TODO: check why the file_path does not have the name of the file
    dbg!(&note);
    let file_exist: bool = Path::new(&note.path).exists();

    if !file_exist {
        let file_creation_result = File::create(&note.path);
        let file_creation = match file_creation_result {
            Ok(_) => println!("File created in {}", &note.path),
            Err(error) => println!("File creation failed: {:?}", error),
        };
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&note.path)
        .unwrap();

    if let Err(e) = writeln!(file, "{:?}", &note.content) {
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
