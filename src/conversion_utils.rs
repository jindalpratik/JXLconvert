use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::Command;

pub fn convert_image(image_path: &Path) -> bool {
    let output = Command::new("cjxl")
        .args(jxl_command(image_path))
        .arg(image_path)
        .arg(image_path.with_extension("jxl"))
        .output();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("conversion.log")
        .expect("Unable to open log file");

    match output {
        Ok(output) => {
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!(
                    "Failed to convert {}: {}",
                    image_path.display(),
                    stderr.trim()
                );
            }

            writeln!(file, "{:?}", output).expect("Unable to write to log file");

            output.status.success()
        }
        Err(err) => {
            eprintln!("Failed to convert {}: {}", image_path.display(), err);
            writeln!(
                file,
                "Failed to execute cjxl for {}: {}",
                image_path.display(),
                err
            )
            .expect("Unable to write to log file");

            false
        }
    }
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
