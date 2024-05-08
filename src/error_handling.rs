use std::fs::File;
use std::io::{Error, ErrorKind, Read};

pub fn error_propagation() {
    let filename = "C:\\Andreea\\newFile.txt";
    match read_from_file(&filename) {
        Ok(text) => {
            println!("read from file txt: {}", text);
        }
        Err(_) => {}
    }
}

fn read_from_file(filename: &str) -> Result<String, Error> {
    let mut file = File::open(filename)?;  //this method can throw an error, if thrown it will be returned " ?;"
    let mut read_text = String::new();
    file.read_to_string(&mut read_text)?; //can throw error
    Ok(read_text)
}


pub fn error_handling() {
    let file_path = "C:\\Andreea\\newFile.json";

    match File::open(file_path) {
        Ok(file) => {
            println!("file was found and opened");
            println!("{:#?}", file);
        }
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => {
                    match File::create(file_path) {
                        Ok(file) => {
                            println!("file created {:#?}", file);
                        }
                        Err(error2) => {
                            println!("{:#?}", error2);
                        }
                    }
                }

                _ => {
                    println!("{:#?}", error);
                }
            }
        }
    }
}