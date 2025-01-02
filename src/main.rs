use jxlconvert::{args_utils, file_utils, path_utils};


fn main() {
    let args = args_utils::Args::get_args();
    if args.source == None {
        println!("Source not provided proceeding with user input")
    }
    let comic_directory = path_utils::get_and_validate_path(args.source, "comics/manga directory");

    file_utils::process_files(comic_directory);
}
