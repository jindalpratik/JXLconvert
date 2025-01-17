use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use walkdir::DirEntry;

pub fn convert_image(image: &DirEntry) {
    let output = Command::new("cjxl")
        .args(jxl_command(&image))
        .arg(image.path().to_str().unwrap())
        .arg(image.path().with_extension("jxl").to_str().unwrap())
        .output()
        .expect(&format!(
            "Failed to convert {}",
            &image.path().to_str().unwrap()
        ));

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("conversion.log")
        .expect("Unable to open log file");

    writeln!(file, "{:?}", output).expect("Unable to write to log file");
}

// Select the jxl command to use for different image types.
fn jxl_command(image: &DirEntry) -> Vec<&str> {
    match image
        .path()
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .map(|s| s.to_lowercase())
    {
        Some(ref ext) if ["jpg", "jpeg"].contains(&ext.as_str()) => vec!["-j", "1"],
        _ => vec!["-d", "0.0"],
    }
}
