use std::any::Any;
use std::error::Error;
use std::fs;
use std::process::Command;

use crate::requirements::{CheckResult, Requirement};

struct KernelCheckResult {
    pub is_supported: bool,
    pub missing_args: Vec<String>,
}

impl CheckResult for KernelCheckResult {
    fn passed(&self) -> bool {
        self.is_supported
    }

    fn log(&self) -> () {
        let result = self.clone();
        if result.is_supported {
            log::info!("Kernel arguments are supported");
        } else {
            log::warn!("Kernel arguments are not supported");
            log::warn!(
                "Missing arguments: {}",
                result.missing_args.join(", ")
            );
        }
    }
}

#[derive(Debug)]
pub struct KernelArgsRequirement;

impl Requirement for KernelArgsRequirement {
    // type CheckResultType = KernelCheckResult;
    fn check(&self) -> Result<Box<dyn CheckResult>, Box<dyn Error>> {
        let result = check_kernel_args()?;
        result.log();
        Ok(Box::new(result))
    }
    fn fix(&self) -> Option<Result<(), Box<dyn Error>>> {
        Some(fix_kernel_args())
    }
    fn fixable(&self) -> bool {
        true
    }
}

fn check_kernel_args() -> Result<KernelCheckResult, Box<dyn Error>> {
    let required_args = ["nomodeset", "nofb", "vga=normal", "console=ttyS0"];
    let cmdline = crate::utils::read_file_to_string("/proc/cmdline");

    let missing_args: Vec<&str> = required_args
        .iter()
        .filter(|&arg| !cmdline.contains(arg))
        .cloned()
        .collect();

    Ok(KernelCheckResult {
        is_supported: missing_args.is_empty(),
        missing_args: missing_args.iter().map(|&arg| arg.to_string()).collect(),
    })
}

fn fix_kernel_args() -> Result<(), Box<dyn Error>> {
    let required_args = "console=ttyS0 vga=normal nofb nomodeset";

    log::info!("Backing up grub");
    fs::copy("/etc/default/grub", "/tmp/grub")?;

    log::info!("Editing grub file");
    /// Finds the line that starts with `GRUB_CMDLINE_LINUX=`
    /// gets the actual value
    /// adds the missing args
    /// panics if the line is not found
    let grub_content = crate::utils::read_file_to_string("/etc/default/grub");

    let grub_content = add_required_args_to_kernel(required_args, grub_content);

    fs::write("/etc/default/grub", grub_content)?;

    log::info!("Backing up grub config");
    fs::copy("/boot/grub/grub.cfg", "/tmp/grub_bkp.cfg")?;

    log::info!("Updating grub config");
    let output = Command::new("update-grub")
        .output()?;

    if !output.status.success() {
        // todo what if this processes fails? maybe an saga pattern to rollback?
        let err_msg = String::from_utf8_lossy(&output.stderr);
        log::error!("Failed to update grub: {}", err_msg);
        return Err("Failed to update grub".into());
    }

    log::warn!("Please reboot the system to apply the kernel changes.");

    Ok(())
}

fn add_required_args_to_kernel(required_args: &str, grub_content: String) -> String {
    let new_grub_content: Vec<String> = grub_content
        .lines()
        .map(|line| {
            if line.starts_with("GRUB_CMDLINE_LINUX=") {
                let grub_cmdline_current_value = line
                    .split('=')
                    .nth(1)
                    .unwrap()
                    .trim_matches('"');
                let grub_cmdline_new_value = add_missing_args(grub_cmdline_current_value, required_args);

                format!("GRUB_CMDLINE_LINUX=\"{}\"", grub_cmdline_new_value)
            } else {
                line.to_string()
            }
        })
        .collect();

    new_grub_content.join("\n")
}

// tests
#[test]
fn test_add_missing_args() {
    let current_value = "console=ttyS0 vga=normal nofb";
    let required_args = "console=ttyS0 vga=normal nofb nomodeset";

    let new_value = add_missing_args(current_value, required_args);

    assert_eq!(new_value, required_args);
}

#[test]
fn test_add_to_empty() {
    let current_value = "";
    let required_args = "console=ttyS0 vga=normal nofb nomodeset";

    let new_value = add_missing_args(current_value, required_args);

    assert_eq!(new_value, required_args);
}

fn add_missing_args(current_value: &str, required_args: &str) -> String {
    let current_value = current_value.trim();
    let mut args: Vec<&str> = current_value.split(' ').collect();
    let required_args: Vec<&str> = required_args.split(' ').collect();

    for arg in required_args {
        if !args.contains(&arg) {
            args.push(arg);
        }
    }

    args.join(" ")
}