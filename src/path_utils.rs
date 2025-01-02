use std::{
    fs::{self},
    path::PathBuf,
};

use dialoguer::Input;

/// Function to get and validate the folder path where the comics are located.
pub fn get_and_validate_path(source: Option<String>, message: &str) -> String {
    // Check if a correct path if passed directly via a flag and use it.
    match source {
        Some(s) => {
            if validate_path(&s) {
                return s;
            } else {
                println!("Provided path is invalid. Please enter a valid path.");
            }
        }
        None => {}
    }

    // Take user input if no correct path passed.
    loop {
        let path_name: String = Input::new()
            .with_prompt(format!("Enter the {}", message))
            .interact_text()
            .unwrap();

        if validate_path(&path_name) {
            return path_name;
        } else {
            println!("Invalid path. Please try again.");
        }
    }
}

pub fn create_temp_dir() -> PathBuf {
    let temp_dir = PathBuf::from(r"./temp_converion_dir");

    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir).unwrap();
    }

    fs::create_dir(&temp_dir).unwrap();

    temp_dir
}

pub fn remove_file(file: &PathBuf, error: &str) {
    fs::remove_file(&file).expect(&format!("Unable to remove file {}", error));
}

pub fn remove_directory(directory: &PathBuf, error: &str) {
    fs::remove_dir_all(&directory).expect(&format!("Unable to remove directory {}", error));
}

fn validate_path(path: &str) -> bool {
    fs::metadata(path).is_ok()
}
