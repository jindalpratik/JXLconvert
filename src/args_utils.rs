use std::thread;

use argh::FromArgs;
use dialoguer::Input;

#[derive(FromArgs)]
/// A Rust CLI app that converts images in comic archives to JPEG XL format.
pub struct Args {
    #[argh(option)]
    /// optional argument to specify comics folder skipping user input.
    pub source: Option<String>,

    #[argh(option)]
    /// optional argument to specify folder skipping user input.
    pub destination: Option<String>,

    #[argh(option)]
    /// inspect a specific ZIP/CBZ file to see its contents and check for image files.
    pub inspect: Option<String>,

    #[argh(option)]
    /// optional argument to specify the number of worker threads to use.
    pub threads: Option<usize>,
}

impl Args {
    pub fn get_args() -> Args {
        argh::from_env()
    }

    pub fn resolve_thread_count(&self) -> usize {
        if let Some(value) = self.threads {
            if value > 0 {
                return value;
            }
            println!("Provided thread count {value} is invalid. Please enter a positive integer.");
        }

        let suggested = thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1usize);

        loop {
            let input: String = Input::new()
                .with_prompt(format!(
                    "How many worker threads should be used? (default {suggested})"
                ))
                .allow_empty(true)
                .interact_text()
                .unwrap();

            if input.trim().is_empty() {
                return suggested.max(1);
            }

            match input.trim().parse::<usize>() {
                Ok(value) if value > 0 => return value,
                _ => println!("Please enter a positive integer."),
            }
        }
    }
}
