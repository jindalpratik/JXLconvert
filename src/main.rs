use jxlconvert::{args_utils, file_utils, path_utils};

fn main() {
    let args = args_utils::Args::get_args();

    let (comic_directory, destination_directory) =
        path_utils::get_and_validate_path(args.source, args.destination);

    file_utils::process_files(comic_directory, destination_directory);
}
