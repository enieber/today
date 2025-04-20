use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Error, Lines, Write};
use std::path::Path;

pub fn new_markdown(file_path: String, content: String) -> Result<String, String> {
    let file = create_file(file_path.as_str());
    match file {
        Ok(mut file_buffer) => {
            let has_write = writeln!(file_buffer, "{}", &content);
            match has_write {
                Ok(_ok) => {
                    return Ok(format!("Line write with success"));
                }
                Err(err) => {
                    return Err(format!("Error to write file: {}", err));
                }
            }
        }
        Err(err) => {
            return Err(format!("Error to open file: {}", err));
        }
    }
}

fn create_file(file_path: &str) -> Result<File, Error> {
    let file = File::create(file_path);
    return file;
}


fn open_file(file_path: &str) -> Result<File, Error> {
    let file = OpenOptions::new().write(true).append(true).open(file_path);
    return file;
}
fn create_file_or_open(file_path: &str) -> Result<File, Error> {
    if Path::new(file_path).exists() {
        open_file(&file_path)
    } else {
        create_file(&file_path)
    }
}
