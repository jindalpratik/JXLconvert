use indicatif::{ProgressBar, ProgressStyle};

pub mod args_utils;
pub mod conversion_utils;
pub mod file_utils;
pub mod path_utils;

pub fn create_progress_bar(len: u64, msg: String) -> ProgressBar {
    // Initialize and configure the progress bar for comics.
    let bar = ProgressBar::new(len);
    bar.set_style(
        ProgressStyle::with_template("[{eta_precise}] {msg} {bar:40.cyan/blue} {pos}/{len}")
            .unwrap()
            .progress_chars("##-"),
    );
    bar.set_message(msg);

    bar
}
