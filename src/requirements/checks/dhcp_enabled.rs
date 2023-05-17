use std::any::Any;
use std::error::Error;
use std::fs;
use std::ops::Deref;

use crate::requirements::{CheckResult, Requirement};

struct DhcpEnabledCheckResult {
    pub is_enabled: bool,
}

impl CheckResult for DhcpEnabledCheckResult {
    fn passed(&self) -> bool {
        self.is_enabled
    }
    fn log(&self) -> () {
        let result = self.deref();
        if result.is_enabled {
            log::info!("DHCP is enabled");
        } else {
            log::warn!("DHCP is not enabled");
        }
    }
}

#[derive(Debug)]
pub struct DhcpEnabledRequirement;

impl Requirement for DhcpEnabledRequirement {
    fn check(&self) -> Result<Box<dyn CheckResult>, Box<dyn Error>> {
        let result = check_dhcp_enabled()?;
        result.log();
        Ok(Box::new(result))
    }
}

/// Checks if DHCP is enabled by looking into the configuration files
/// located in the `/etc/netplan/` directory. If the string `dhcp4: true`
/// is found in one of the files, it assumes DHCP is enabled.
fn check_dhcp_enabled() -> Result<DhcpEnabledCheckResult, Box<dyn Error>> {
    let directory = "/etc/netplan/";

    // read the directory
    let config_files = fs::read_dir(directory)
        .map_err(|_| format!("Failed to read directory: {}", directory))?;

    let mut dhcp_enabled = false;

    for file in config_files {
        // read the file
        let file = file.map_err(|_| "Failed to read file")?;

        // read the file content
        let content = fs::read_to_string(file.path())
            .map_err(|_| format!("Failed to read file content: {:?}", file.path()))?;

        if content.contains("dhcp4: true") {
            dhcp_enabled = true;
            break;
        }
    }

    Ok(DhcpEnabledCheckResult { is_enabled: dhcp_enabled })
}

