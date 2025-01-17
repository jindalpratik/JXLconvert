use std::path::{Path, PathBuf};

use walkdir::{DirEntry, WalkDir};
use zip_extensions::{zip_create_from_directory, zip_extract};

use crate::{
    conversion_utils, create_progress_bar,
    path_utils::{self},
};

pub fn process_files(comic_directory: String, destination_directory: Option<String>) -> () {
    let comic_files = filter_comic_files(&comic_directory);

    // Initialize and configure the progress bar for comics.
    let comic_progress_bar =
        create_progress_bar(comic_files.len() as u64, "Comics Processed".to_string());

    for comic in comic_files {
        comic_progress_bar.inc(1);

        // Create temp directory and empty it if already exists.
        let temp_dir = path_utils::create_temp_dir();

        let comic_pathbuf = comic.into_path();

        zip_extract(&comic_pathbuf, &temp_dir).expect(&format!(
            "Unable to extract comic file, {}",
            &comic_pathbuf.to_str().unwrap()
        ));

        process_images(&temp_dir);

        // Create destination file path from comic file path based on destination directory.
        let dest_path = match destination_directory {
            Some(ref destination_directory) => {
                let relative_path = comic_pathbuf.strip_prefix(&comic_directory).unwrap();
                let dest_path = Path::new(&destination_directory).join(relative_path);
                if let Some(parent) = dest_path.parent() {
                    std::fs::create_dir_all(parent).expect("Failed to create destination directories");
                }
                dest_path
            }
            None => comic_pathbuf.clone(),
        };

        // Remove original comic file if original comic file is not specified.
        if destination_directory.is_none() {
            path_utils::remove_file(&comic_pathbuf, comic_pathbuf.to_str().unwrap());
        }

        zip_create_from_directory(&dest_path, &temp_dir).unwrap();

        path_utils::remove_directory(&temp_dir.to_path_buf(), &temp_dir.to_str().unwrap());
    }

    comic_progress_bar.finish_and_clear();
}

fn process_images(images_directory: &PathBuf) -> () {
    let image_files = filter_image_files(&images_directory);

    let image_progress_bar = create_progress_bar(
        image_files.len() as u64,
        "Comic Images Processed".to_string(),
    );

    for image in image_files {
        image_progress_bar.inc(1);
        conversion_utils::convert_image(&image);

        // Delete the original image file after conversion.
        path_utils::remove_file(&image.path().to_path_buf(), &image.path().to_str().unwrap());
    }

    image_progress_bar.finish_and_clear();
}

/*
Walk through the comic directory and create a vector of all the comic files.
*/
fn filter_comic_files(comic_directory: &String) -> Vec<DirEntry> {
    let comic_files = WalkDir::new(comic_directory)
        .into_iter()
        // Filter the comic files out of the directories and unrelated files
        .filter(|e| {
            e.as_ref()
                .unwrap()
                .metadata()
                .expect("Unable to detect comic files")
                .is_file()
                && is_comic(e.as_ref().unwrap())
        })
        // Unwrap the directory entry and map it to the resulting vector
        .map(|e| e.unwrap())
        .collect();

    comic_files
}

/*
Check if a file is a comic or not
*/
fn is_comic(file: &DirEntry) -> bool {
    matches!(
        file.path().extension().and_then(std::ffi::OsStr::to_str).map(|s| s.to_lowercase()),
        Some(ref ext) if ["cbz"].contains(&ext.as_str())
    )
}

/*
Walk through the unzipped comic and filter out all the images out of it.
Also takes PathBuf instead of String as temp_dir is already stored in PathBuf.
*/
fn filter_image_files(images_dir: &PathBuf) -> Vec<DirEntry> {
    let image_files = WalkDir::new(images_dir)
        .into_iter()
        .filter(|e| {
            e.as_ref()
                .unwrap()
                .metadata()
                .expect("Unable to detect image file")
                .is_file()
                && is_image(e.as_ref().unwrap())
        })
        .map(|e| e.unwrap())
        .collect();

    image_files
}

/*
Checks if the given file is an accepted image file or not.
*/
fn is_image(file: &DirEntry) -> bool {
    matches!(
        file.path().extension().and_then(std::ffi::OsStr::to_str).map(|s| s.to_lowercase()),
        // TODO: Add more image formats to the list
        Some(ref ext) if ["jpg", "jpeg", "png", "gif"].contains(&ext.as_str())
    )
}
