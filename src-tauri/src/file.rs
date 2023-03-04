use std::fs::{self, File};
use std::io::Write;

#[tauri::command]
pub fn save_to_file(data: String) -> String {
    let status: String;
    let file_name = "save.txt";
    let file = File::create(file_name);

    match file {
        Ok(mut file) => {
            let is_written = writeln!(file, "{}", data);

            match is_written {
                Ok(_) => {
                    status = String::from("Success");
                }
                Err(_) => {
                    status = String::from("Error: Unable save to file");
                }
            }
        }
        Err(_) => {
            status = String::from("Error: Unable to create a file");
        }
    }

    status
}

#[tauri::command]
pub fn load_from_file() -> String {
    let status: String;
    let file_name = "save.txt";
    let data = fs::read_to_string(file_name);

    match data {
        Ok(data) => {
            status = data;
        }
        Err(_) => {
            status = String::from("Error: Unable to load from a file");
        }
    }

    status
}
