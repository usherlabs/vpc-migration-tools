use std::error::Error;
use dialoguer::Confirm;
use crate::requirements::{CheckResult, checks, Requirement};

pub fn run_requirements() -> Result<(), Box<dyn Error>> {
    log::info!("");
    log::info!("The requirements are based on the documentation available at https://cloud.ibm.com/docs/vpc?topic=vpc-create-linux-custom-image");

    // let vitio_check = checks::virtio_drivers::VitioDriversRequirement.check()?;
    // let os_support_check = checks::os_support::OSSupportRequirement.check()?;
    // let cloud_init_check = checks::cloud_init::CloudInitRequirement.check()?;
    // let boot_disk_size_check = checks::boot_disk_size::BootDiskSizeRequirement.check()?;
    // let kernel_args_check = checks::kernel_args::KernelArgsRequirement.check()?;
    // let dhcp_enabled_check = checks::dhcp_enabled::DhcpEnabledRequirement.check()?;
    //
    // let all_checks: Vec<( Box<dyn CheckResult>,  )> = vec![
    //     Box::new(vitio_check),
    //     Box::new(os_support_check),
    //     Box::new(cloud_init_check),
    //     Box::new(boot_disk_size_check),
    //     Box::new(kernel_args_check),
    //     Box::new(dhcp_enabled_check),
    // ];

    let requirements: Vec<Box<dyn Requirement>> = vec![
        Box::new(checks::os_support::OSSupportRequirement),
        Box::new(checks::cloud_init::CloudInitRequirement),
        Box::new(checks::virtio_drivers::VitioDriversRequirement),
        Box::new(checks::boot_disk_size::BootDiskSizeRequirement),
        Box::new(checks::dhcp_enabled::DhcpEnabledRequirement),
        Box::new(checks::kernel_args::KernelArgsRequirement),
    ];


    requirements
        .into_iter()
        .map(|requirement| (requirement.check(), requirement))
        .filter_map(|(result, requirement)| match result {
            Ok(res) if !res.passed() => Some((res, requirement)),
            _ => None,
        })
        .filter(|(result, requirement)| filter_fixable(requirement.as_ref()))
        .for_each(|(result, fixable_requirement)| {
            log::info!("\nRequirement {:?} failed. Available fix:", fixable_requirement);

            if Confirm::new()
                .with_prompt("Do you want to apply this fix?")
                .interact()
                .unwrap_or(false)
            {
                match fixable_requirement.fix() {
                    Some(Ok(_)) => log::info!("Fix applied successfully"),
                    Some(Err(e)) => log::error!("Error applying fix: {:?}", e),
                    None => log::info!("No fix available"),
                }
            }
        });

    Ok(())
}

fn filter_fixable(requirement: &dyn Requirement) -> bool {
    requirement.fixable()
}
