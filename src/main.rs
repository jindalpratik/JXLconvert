use jxlconvert::{file_utils, path_utils};

fn main() {
    let comic_directory = path_utils::get_and_validate_path("comics/manga directory");

    file_utils::process_files(comic_directory);
}
