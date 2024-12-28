use indicatif::{ProgressBar, ProgressStyle};
use walkdir::{DirEntry, WalkDir};

pub fn process_files(comic_directory: String) -> () {
    let comic_files = filter_comic_files(comic_directory);

    // Initialize and configure the progress bar.
    let bar = ProgressBar::new(comic_files.len() as u64);
    bar.set_style(
        ProgressStyle::with_template(
            "[{eta_precise}] {msg} {bar:40.cyan/blue} {pos}/{len}",
        )
        .unwrap()
        .progress_chars("##-"),
    );
    bar.set_message("Comics Processed");

    for _ in comic_files {
    }

    bar.finish();
}

/*
Walk through the comic directory and create a vector of all the comic files.
*/
fn filter_comic_files(comic_directory: String) -> Vec<DirEntry> {
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
        Some(ref ext) if ext == "cbz" || ext == "cbr"
    )
}
