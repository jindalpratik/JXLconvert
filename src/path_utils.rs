use std::{
    fs::{self},
    path::PathBuf,
};

use dialoguer::{Input, Select};

/// Function to get and validate the folder path where the comics are located.
pub fn get_and_validate_path(
    source: Option<String>,
    destination: Option<String>,
) -> (String, Option<String>) {
    // Check if a correct path if passed directly via a flag and use it.
    let source: String = match source {
        Some(s) => {
            if validate_path(&s) {
                s
            } else {
                println!("Provided path {s} is invalid. Please enter a valid path.");
                get_path("source comic directory")
            }
        }
        None => get_path("source comic directory"),
    };

    let destination: Option<String> = match destination {
        // Destination provided as an argument.
        Some(d) => {
            if validate_path(&d) {
                Some(d)
            } else {
                println!("Provided path {d} is invalid. Please enter a valid path.");
                Some(get_path("destination comic directory"))
            }
        }

        // No Destination provided in argument.
        None => {
            let items = vec!["YES", "NO"];

            let selection = Select::new()
                .with_prompt("Do you want to overwrite existing comics? [Proceed with caution]")
                .items(&items)
                .default(1)
                .interact()
                .unwrap();

            if items[selection] == "YES" {
                None
            } else {
                Some(get_path("destination comic directory"))
            }
        }
    };

    (source, destination)
}

fn get_path(msg: &str) -> String {
    // Take user input if no correct path passed.
    loop {
        let path_name: String = Input::new()
            .with_prompt(format!("Enter the {}", msg))
            .interact_text()
            .unwrap();

        if validate_path(&path_name) {
            return path_name;
        } else {
            println!("Invalid path. Please try again.");
        }
    }
}

fn validate_path(path: &str) -> bool {
    fs::metadata(path).is_ok()
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
