use std::fs::File;
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};

use rayon::prelude::*;
use walkdir::{DirEntry, WalkDir};
use zip::write::SimpleFileOptions;
use zip::{ZipArchive, ZipWriter};

use crate::{
    conversion_utils, create_progress_bar,
    path_utils::{self},
};

/// Extract a zip file to a destination directory
fn extract_zip(zip_path: &Path, dest_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = dest_dir.join(file.mangled_name());

        if file.is_dir() {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent()
                && !p.exists()
            {
                std::fs::create_dir_all(p)?;
            }
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}

/// Create a zip file from a directory
fn create_zip_from_directory(
    zip_path: &Path,
    source_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(zip_path)?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let walkdir = WalkDir::new(source_dir);
    let it = walkdir.into_iter();

    for entry in it {
        let entry = entry?;
        let path = entry.path();
        let name = path.strip_prefix(source_dir)?;

        // Write file or directory explicitly
        // Some unzip tools unpack files with directory paths correctly, others don't!
        if path.is_file() {
            zip.start_file(name.to_string_lossy(), options)?;
            let mut f = File::open(path)?;
            io::copy(&mut f, &mut zip)?;
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mkdir failure on unzip
            zip.add_directory(name.to_string_lossy(), options)?;
        }
    }
    zip.finish()?;
    Ok(())
}

/// Check if a ZIP file contains any image files
fn zip_contains_images(zip_path: &Path) -> Result<bool, Box<dyn std::error::Error>> {
    let file = File::open(zip_path)?;
    let reader = BufReader::new(file);
    let mut archive = ZipArchive::new(reader)?;

    for i in 0..archive.len() {
        let file = archive.by_index(i)?;

        // Skip directories
        if file.is_dir() {
            continue;
        }

        // Get the file path
        let outpath = match file.enclosed_name() {
            Some(path) => path,
            None => {
                continue;
            }
        };

        // Check if it's an image file using the is_image_path function
        if is_image_path(&outpath) {
            return Ok(true);
        }
    }

    Ok(false)
}

pub fn process_files(comic_directory: String, destination_directory: Option<String>) {
    let comic_files = filter_comic_files(&comic_directory);

    // Initialize and configure the progress bar for comics.
    let comic_progress_bar =
        create_progress_bar(comic_files.len() as u64, "Comics Processed".to_string());

    for comic in comic_files {
        comic_progress_bar.inc(1);

        let comic_pathbuf = comic.into_path();

        // Check if the ZIP file contains any image files before processing
        match zip_contains_images(&comic_pathbuf) {
            Ok(contains_images) => {
                if !contains_images {
                    continue;
                }
            }
            Err(_) => {
                continue;
            }
        }

        // Create temp directory and empty it if already exists.
        let temp_dir = path_utils::create_temp_dir();

        extract_zip(&comic_pathbuf, &temp_dir).unwrap_or_else(|_| {
            panic!(
                "Unable to extract comic file, {}",
                &comic_pathbuf.to_str().unwrap()
            )
        });

        process_images(&temp_dir);

        // Create destination file path from comic file path based on destination directory.
        let dest_path = match destination_directory {
            Some(ref destination_directory) => {
                let relative_path = comic_pathbuf.strip_prefix(&comic_directory).unwrap();
                let dest_path = Path::new(&destination_directory).join(relative_path);
                if let Some(parent) = dest_path.parent() {
                    std::fs::create_dir_all(parent)
                        .expect("Failed to create destination directories");
                }
                dest_path
            }
            None => comic_pathbuf.clone(),
        };

        // Remove original comic file if original comic file is not specified.
        if destination_directory.is_none() {
            let comic_display = comic_pathbuf.display().to_string();
            path_utils::remove_file(comic_pathbuf.as_path(), &comic_display);
        }

        create_zip_from_directory(&dest_path, &temp_dir).unwrap();

        let temp_display = temp_dir.display().to_string();
        path_utils::remove_directory(temp_dir.as_path(), &temp_display);
    }

    comic_progress_bar.finish_and_clear();
}

fn process_images(images_directory: &PathBuf) {
    let image_files = filter_image_files(images_directory);

    let image_progress_bar = create_progress_bar(
        image_files.len() as u64,
        "Comic Images Processed".to_string(),
    );

    if image_files.is_empty() {
        image_progress_bar.finish_and_clear();
        return;
    }

    let progress = image_progress_bar.clone();

    image_files.into_par_iter().for_each(|image| {
        let image_path = image.into_path();
        let image_display = image_path.display().to_string();

        let converted = conversion_utils::convert_image(image_path.as_path());

        if converted {
            path_utils::remove_file(image_path.as_path(), &image_display);
        }

        progress.inc(1);
    });

    image_progress_bar.finish_and_clear();
}

/*
Walk through the comic directory and create a vector of all the comic files.
*/
fn filter_comic_files(comic_directory: &String) -> Vec<DirEntry> {
    WalkDir::new(comic_directory)
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
        .collect()
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
    WalkDir::new(images_dir)
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
        .collect()
}

/*
Helper function to check if a path has an image extension
*/
fn is_image_path(path: &Path) -> bool {
    matches!(
        path.extension().and_then(std::ffi::OsStr::to_str).map(|s| s.to_lowercase()),
        // TODO: Add more image formats to the list
        Some(ref ext) if ["jpg", "jpeg", "png", "gif"].contains(&ext.as_str())
    )
}

/*
Checks if the given file is an accepted image file or not.
*/
fn is_image(file: &DirEntry) -> bool {
    is_image_path(file.path())
}
