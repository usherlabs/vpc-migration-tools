use std::any::Any;
use std::error::Error;
use std::ops::Deref;
use std::process::Command;

use crate::requirements::{CheckResult, Requirement};

struct CloudInitCheckResult {
    pub is_installed: bool,
}

impl CheckResult for CloudInitCheckResult {
    fn passed(&self) -> bool {
        self.is_installed
    }
    fn log(&self) -> () {
        let result = self.deref();
        if result.is_installed {
            log::info!("Cloud-init is installed");
        } else {
            log::warn!("Cloud-init is not installed");
        }
    }
}

#[derive(Debug)]
pub struct CloudInitRequirement;

impl Requirement for CloudInitRequirement {
    fn check(&self) -> Result<Box<dyn CheckResult>, Box<dyn Error>> {
        let result = check_cloud_init()?;
        result.log();
        Ok(Box::new(result))
    }
}

fn check_cloud_init() -> Result<CloudInitCheckResult, Box<dyn Error>> {
    let mut cmd = Command::new("cloud-init");
    cmd.arg("--version");


    Ok(CloudInitCheckResult {
        is_installed: cmd.status().is_ok()
    })
}


