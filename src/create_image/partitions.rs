use std::error::Error;
use std::process::Command;
use crate::utils;

pub fn list_available_devices() -> Result<Vec<utils::Option>, Box<dyn Error>> {
    let initial_list = Command::new("lsblk")
        .arg("-d")
        .arg("-o")
        .arg("NAME,TYPE")
        .output()?;

    if !initial_list.status.success() {
        let err_msg = String::from_utf8_lossy(&initial_list.stderr);
        return Err(format!("Failed to list available devices: {}", err_msg).into());
    }

    let initial_list = String::from_utf8_lossy(&initial_list.stdout);
    let mut devices = Vec::new();

    for line in initial_list.lines() {
        if line.starts_with("NAME") {
            continue;
        }

        let mut parts = line.split_whitespace();
        let name = parts.next().unwrap();
        let device_type = parts.next().unwrap();

        if device_type == "disk" {
            devices.push(utils::Option {
                label: name.to_string(),
                value: format!("/dev/{}", name),
            });
        }
    }

    Ok(devices)
}



#[test]
fn test_list_available_disks() {
    let devices = list_available_devices().unwrap();
    assert!(devices.len() > 0);
    for device in devices {
        println!("{}: {}", device.label, device.value);
    }
}