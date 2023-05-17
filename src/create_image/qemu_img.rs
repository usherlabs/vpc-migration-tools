use std::error::Error;
use std::process::{Command, Stdio};

pub fn is_installed() -> bool {
    Command::new("qemu-img")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

pub fn install() -> Result<(), Box<dyn Error>> {
    let output = Command::new("sudo")
        .arg("apt-get")
        .arg("update")
        .output()?;

    if !output.status.success() {
        return Err("Failed to update apt-get".into());
    }

    let output = Command::new("sudo")
        .arg("apt-get")
        .arg("install")
        .arg("-y")
        .arg("qemu-utils")
        .output()?;

    if output.status.success() {
        log::info!("qemu-utils is installed");
        Ok(())
    } else {
        Err("Failed to install qemu-utils".into())
    }
}

pub struct ImageDetails {
    pub filepath: String,
}
pub fn create_image(image_name: &str, dir: &str, device: &str) -> Result<ImageDetails, Box<dyn Error>> {
    let file_path = format!("{}/{}.qcow2", dir, image_name);

    let output = Command::new("sudo")
        .arg("qemu-img")
        .arg("convert")
        .arg("-c") // compact
        .arg("-p") // Show progress
        .arg("-O")
        .arg("qcow2")
        .arg(device)
        .arg(&file_path)
        .output()?;

    if output.status.success() {
        Ok(ImageDetails {
            filepath: file_path,
        })
    } else {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to create image: {}", err_msg).into())
    }
}

pub fn check_image(image_path: &str) -> Result<(), Box<dyn Error>> {
    let output = Command::new("sudo")
        .arg("qemu-img")
        .arg("check")
        .arg(image_path)
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to check image: {}", err_msg).into())
    }
}
