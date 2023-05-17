use std::error::Error;

use crate::create_image::{free_space, qemu_img};

pub struct Options {
    pub skip_free_space: Option<bool>,
    pub image_name: String,
    pub dir: String,
    pub device: String,
}

///Validate the image_name and dir inputs.
/// - Check if qemu-img is installed and install it if not.
/// - Check if there are any file conflicts.
/// - Create and erase the empty space.
/// - Create the image with qemu-img.
/// - Check the image.
pub fn run(options: Options) -> Result<(), Box<dyn Error>> {
    let Options {
        skip_free_space,
        image_name,
        dir,
        device,
    } = options;

    log::info!("Creating image {} in {}", image_name, dir);

    // Validate inputs
    log::info!("Validating inputs...");
    if !std::fs::metadata(&dir).map(|m| m.is_dir()).unwrap_or(false) {
        return Err(format!("Invalid directory: {}", dir).into());
    }


    // Check if qemu-img is installed
    log::info!("Checking if qemu-img is installed...");
    if !qemu_img::is_installed() {
        log::info!("qemu-img is not installed, installing...");
        qemu_img::install()?;
    }

    // Check if there's any existent file that will have conflict with our creation steps
    log::info!("Checking for conflicts...");
    free_space::check_conflict(&dir, &image_name)?;

    // Create and erase the empty space of the machine
    if let Some(true) = skip_free_space {
        log::info!("Skipping free space creation...");
    } else {
        let empty_file_path = "/tmp/emptyfile".to_string();
        log::info!("Creating empty file...");
        log::info!("The intention is to fill up the empty space of the machine");
        free_space::create_empty_file(&empty_file_path)?;
        log::info!("Erasing empty file...");
        free_space::erase_empty_file(&empty_file_path)?;
    }

    // Create the image with qemu-img
    log::info!("Creating image...");
    let image = qemu_img::create_image(&image_name, &dir, &device)?;
    log::info!("Image created at {}", image.filepath);

    // Check the image
    log::info!("Checking image...");
    qemu_img::check_image(&image.filepath)?;

    log::info!("Image created successfully");

    Ok(())
}

