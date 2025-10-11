use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::Command;

pub fn convert_image(image_path: &Path) {
    let output = Command::new("cjxl")
        .args(jxl_command(image_path))
        .arg(image_path)
        .arg(image_path.with_extension("jxl"))
        .output()
        .unwrap_or_else(|err| panic!("Failed to convert {}: {}", image_path.display(), err));

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("conversion.log")
        .expect("Unable to open log file");

    writeln!(file, "{:?}", output).expect("Unable to write to log file");
}

// Select the jxl command to use for different image types.
fn jxl_command(image_path: &Path) -> Vec<&'static str> {
    match image_path
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .map(|s| s.to_lowercase())
    {
        Some(ref ext) if ["jpg", "jpeg"].contains(&ext.as_str()) => vec!["-j", "1"],
        _ => vec!["-d", "0.0"],
    }
}
