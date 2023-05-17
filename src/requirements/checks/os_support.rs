use std::any::Any;
use std::error::Error;
use std::ops::Deref;

use sysinfo::{System, SystemExt};

use crate::requirements::{CheckResult, Requirement};

#[derive(Debug)]
pub struct OSSupportRequirement;

impl Requirement for OSSupportRequirement {
    fn check(&self) -> Result<Box<dyn CheckResult>, Box<dyn Error>> {
        let result = check_os_support()?;
        result.log();
        Ok(Box::new(result))
    }
}

struct OsCheckResult {
    pub os_name: String,
    pub os_version: String,
    pub is_supported: bool,
}


impl CheckResult for OsCheckResult {
    fn passed(&self) -> bool {
        self.is_supported
    }
    fn log(&self) {
        let result = self.deref();
        if result.is_supported {
            log::info!("{}, {} is supported", result.os_name, result.os_version);
        } else {
            log::error!("{}, {} is not supported", result.os_name, result.os_version);
        }
    }
}


fn check_os_support() -> Result<OsCheckResult, Box<dyn Error>> {
    // Supported OS list
    let supported_os = [
        ("debian", vec!["10", "11"]),
        ("rhel", vec!["7", "8", "9"]),
        ("rocky", vec!["8.5"]),
        ("suse", vec!["12", "15"]),
        ("ubuntu", vec!["18.04", "20.04", "22.04"]),
        ("windows", vec!["2012", "2012 R2", "2016", "2019", "2022"]),
        ("fedora", vec![]),  // Fedora Core OS has no version
    ];

    let mut sys = System::new();
    sys.refresh_system();

    let os_name = sys.name().expect("Failed to get OS Name").to_lowercase();
    let os_version = sys.os_version().expect("Failed to get OS Version").to_lowercase();

    // instead of printing the about compatibility, it would be better to return os name, os version, and a boolean
    for (name, versions) in &supported_os {
        if os_name == *name && versions.contains(&os_version.as_str()) {
            return Ok(OsCheckResult {
                os_name,
                os_version,
                is_supported: true,
            });
        }
    }

    Ok(OsCheckResult {
        os_name,
        os_version,
        is_supported: false,
    })
}


