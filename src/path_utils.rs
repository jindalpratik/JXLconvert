use std::{
    fs::{self},
    path::PathBuf,
};

use dialoguer::Input;

pub fn get_and_validate_path(message: &str) -> String {
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
