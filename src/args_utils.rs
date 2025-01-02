use argh::FromArgs;

#[derive(FromArgs)]
/// A Rust CLI app that converts images in comic archives to JPEG XL format.
pub struct Args {
    #[argh(option)]
    /// optional argument to specify comics folder skipping user input.
    pub source: Option<String>,
}

impl Args {
    pub fn get_args() -> Args {
        argh::from_env()
    }
}
