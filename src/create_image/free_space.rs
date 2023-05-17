use std::error::Error;
use std::path::Path;
use std::process::Command;

pub fn check_conflict(dir: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    let file_path = format!("{}/{}", dir, file_name);

    if Path::new(&file_path).exists() {
        return Err(format!("Conflict: a file named {} already exists in {}", file_name, dir).into());
    }

    Ok(())
}

pub fn create_empty_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    let output = Command::new("sudo")
        .arg("dd")
        .arg("if=/dev/zero")
        .arg(format!("of={}", file_path))
        .arg("status=progress") // Show progress
        .arg("bs=1M")
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        // if the is of kind "No space left on device", there's no problem, the intention is really to fill everything
        if err_msg.contains("No space left on device") {
            Ok(())
        } else {
            Err(format!("Failed to create empty file: {}", err_msg).into())
        }
    }
}

pub fn erase_empty_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    let output = Command::new("sudo")
        .arg("rm")
        .arg(file_path)
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to erase empty file: {}", err_msg).into())
    }
}
