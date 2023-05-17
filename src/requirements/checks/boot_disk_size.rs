use std::any::Any;
use std::error::Error;
use std::ops::Deref;

use sysinfo::{DiskExt, SystemExt};

use crate::requirements::{CheckResult, Requirement};

struct BootDiskSizeCheckResult {
    pub is_supported: bool,
    acual_size_in_gb: u64,
}

impl CheckResult for BootDiskSizeCheckResult {
    fn passed(&self) -> bool {
        self.is_supported
    }
    fn log(&self)  {
        let result = self.deref();
        if result.is_supported {
            log::info!("Boot disk size is supported");
        } else {
            log::warn!("Boot disk size is not supported, limit is 250 GB, and the actual size is {} GB", result.acual_size_in_gb);
        }
    }
}

#[derive(Debug)]
pub struct BootDiskSizeRequirement;

impl Requirement for BootDiskSizeRequirement {
    fn check(&self) -> Result<Box<dyn CheckResult>, Box<dyn Error>> {
        let result = check_boot_disk_size();
        result.log();
        Ok(Box::new(result))
    }
}

fn check_boot_disk_size() -> BootDiskSizeCheckResult {
    let mut sys = sysinfo::System::new();
    sys.refresh_disks();

    let disk_sum = sys
        .disks()
        .iter()
        .map(|disk| disk.total_space())
        .sum::<u64>();

    let disk_size_gb = disk_sum / 1024 / 1024 / 1024; // Convert to GB

    if disk_size_gb < 10 {
        log::warn!(
            "Boot disk size is less than 10 GB. This works, however, it'll be rounded up to 10 GB.",
        );
    }

    BootDiskSizeCheckResult {
        is_supported: disk_size_gb < 250,
        acual_size_in_gb: disk_size_gb,
    }
}
