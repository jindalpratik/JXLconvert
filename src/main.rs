use jxlconvert::{args_utils, file_utils, path_utils};
use rayon::ThreadPoolBuilder;

fn main() {
    let args = args_utils::Args::get_args();

    let thread_count = args.resolve_thread_count();

    if let Err(err) = ThreadPoolBuilder::new()
        .num_threads(thread_count)
        .build_global()
    {
        eprintln!(
            "Unable to configure global thread pool ({}). Falling back to Rayon defaults.",
            err
        );
    }

    println!("Using {thread_count} worker thread(s).");

    let (comic_directory, destination_directory) =
        path_utils::get_and_validate_path(args.source, args.destination);

    file_utils::process_files(comic_directory, destination_directory);
}
