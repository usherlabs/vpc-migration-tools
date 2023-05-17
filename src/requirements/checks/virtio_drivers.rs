use std::any::Any;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Deref;
use std::process::Command;

use thiserror::Error;

use crate::requirements::{CheckResult, Requirement};
use crate::utils;

#[derive(Debug)]
pub struct VitioDriversRequirement;

impl Requirement for VitioDriversRequirement {
    fn check(&self) -> Result<Box<dyn CheckResult>, Box<dyn Error>> {
        let result = check_virtio_drivers()?;
        result.log();
        Ok(Box::new(result))
    }
}

struct VirtioDriversCheckResult {
    passed: bool,
}

impl CheckResult for VirtioDriversCheckResult {
    fn passed(&self) -> bool {
        self.passed
    }
    fn log(&self) {
        let result = self.deref();
        if result.passed {
            log::info!("VIRTIO drivers are present");
        } else {
            log::warn!("VIRTIO drivers are not present");
        }
    }
}

/// Checks if the VIRTIO_BLK and VIRTIO_NET drivers are present in the kernel.
/// It reads the kernel configuration file located in `/boot/config-<kernel_version>`
/// and checks if the strings `VIRTIO_BLK` and `VIRTIO_NET` are present.
fn check_virtio_drivers() -> Result<VirtioDriversCheckResult, Box<dyn Error>> {
    let kernel_version = get_kernel_version();
    let config_file = format!("/boot/config-{}", kernel_version?);
    let config_content = utils::read_file_to_string(&config_file);

    let passed = config_content.contains("VIRTIO_BLK") && config_content.contains("VIRTIO_NET");

    Ok(VirtioDriversCheckResult { passed })
}

#[derive(Error, Debug)]
enum KernelVersionError {
    #[error("Command execution failed")]
    CommandExecutionFailed(#[from] std::io::Error),
    #[error("Command returned non-zero status: {0}")]
    NonZeroExitStatus(i32),
}

fn get_kernel_version() -> Result<String, KernelVersionError> {
    let output = Command::new("uname")
        .arg("-r")
        .output()?;

    if !output.status.success() {
        return Err(KernelVersionError::NonZeroExitStatus(output.status.code().unwrap_or(-1)));
    }

    let kernel_version = String::from_utf8_lossy(&output.stdout);
    Ok(kernel_version.trim().to_string())
}