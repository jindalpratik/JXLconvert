use std::fs;

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


fn validate_path(path: &str) -> bool {
    fs::metadata(path).is_ok()
}
