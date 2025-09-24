use argh::FromArgs;

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
}

impl Args {
    pub fn get_args() -> Args {
        argh::from_env()
    }
}
